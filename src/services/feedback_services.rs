use crate::{
    dto::feedback::{
        CreateFeedbackRequest, FeedbackListQuery, FeedbackResponse, UpdateFeedbackRequest,
    },
    errors::AppError,
    models::Feedback,
    repositories::FeedbackRepository,
};

pub struct FeedbackService {
    feedback_repo: FeedbackRepository,
}

impl FeedbackService {
    pub fn new(feedback_repo: FeedbackRepository) -> Self {
        Self { feedback_repo }
    }

    pub async fn list(
        &self,
        query: FeedbackListQuery,
    ) -> Result<(Vec<FeedbackResponse>, i64, u32, u32), AppError> {
        let page = query.page.unwrap_or(1).max(1);
        let limit = query.limit.unwrap_or(10).min(100).max(1);

        let (items, total) = self.feedback_repo.find_all_public(page, limit).await?;

        let responses = items
            .into_iter()
            .map(|(f, user_name)| feedback_to_response(f, user_name))
            .collect();

        Ok((responses, total, page, limit))
    }

    pub async fn get(&self, id: &str) -> Result<FeedbackResponse, AppError> {
        let (feedback, user_name) = self
            .feedback_repo
            .find_public_by_id(id)
            .await?
            .ok_or_else(|| AppError::NotFound("Feedback not found".to_string()))?;

        Ok(feedback_to_response(feedback, user_name))
    }

    pub async fn create(
        &self,
        user_id: &str,
        req: CreateFeedbackRequest,
    ) -> Result<FeedbackResponse, AppError> {
        let id = Feedback::new_id();
        let is_public = req.is_public.unwrap_or(true);

        let feedback = self
            .feedback_repo
            .create(&id, user_id, &req.message, is_public)
            .await?;

        // For the response, we need the user's name - fetch from a join or use a placeholder
        // Re-fetch via the public query if it's public, otherwise construct manually
        let user_name = self
            .get_user_name_for_feedback(&feedback.id)
            .await
            .unwrap_or_default();

        Ok(feedback_to_response(feedback, user_name))
    }

    pub async fn update(
        &self,
        id: &str,
        user_id: &str,
        req: UpdateFeedbackRequest,
    ) -> Result<FeedbackResponse, AppError> {
        let feedback = self
            .feedback_repo
            .find_by_id(id)
            .await?
            .ok_or_else(|| AppError::NotFound("Feedback not found".to_string()))?;

        if feedback.user_id != user_id {
            return Err(AppError::Forbidden(
                "You do not have permission to update this feedback".to_string(),
            ));
        }

        let updated = self
            .feedback_repo
            .update(id, req.message.as_deref(), req.is_public)
            .await?;

        let user_name = self
            .get_user_name_for_feedback(&updated.id)
            .await
            .unwrap_or_default();

        Ok(feedback_to_response(updated, user_name))
    }

    pub async fn delete(&self, id: &str, user_id: &str) -> Result<(), AppError> {
        let feedback = self
            .feedback_repo
            .find_by_id(id)
            .await?
            .ok_or_else(|| AppError::NotFound("Feedback not found".to_string()))?;

        if feedback.user_id != user_id {
            return Err(AppError::Forbidden(
                "You do not have permission to delete this feedback".to_string(),
            ));
        }

        self.feedback_repo.soft_delete(id).await?;

        Ok(())
    }

    async fn get_user_name_for_feedback(&self, feedback_id: &str) -> Result<String, AppError> {
        // Try to find via public query to get name
        if let Some((_, name)) = self.feedback_repo.find_public_by_id(feedback_id).await? {
            return Ok(name);
        }
        // Fall back to empty string for non-public feedback responses
        Ok(String::new())
    }
}

fn feedback_to_response(f: Feedback, user_name: String) -> FeedbackResponse {
    FeedbackResponse {
        id: f.id,
        user_name,
        message: f.message,
        created_at: f.created_at,
    }
}
