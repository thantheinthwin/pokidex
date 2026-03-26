package handlers

import (
	"net/http"

	"github.com/gin-gonic/gin"
)

func Health(version string) gin.HandlerFunc {
	return func(c *gin.Context) {
		c.JSON(http.StatusOK, gin.H{"status": "ok", "version": version})
	}
}
