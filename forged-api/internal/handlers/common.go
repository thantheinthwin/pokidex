package handlers

import (
	"net/http"

	"github.com/gin-gonic/gin"
)

func respondOK(c *gin.Context) {
	_, _ = c.Get("user_id")
	c.JSON(http.StatusOK, gin.H{"status": "ok"})
}
