// Logout is stateless with JWT - token invalidation would require a token blacklist.
// For this implementation, logout is handled at the handler level by instructing the client
// to drop the token. This file exists for architectural completeness.
