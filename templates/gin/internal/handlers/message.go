package handlers

import (
	"net/http"

	"github.com/gin-gonic/gin"
)

func GetMessage(c *gin.Context) {
	c.JSON(http.StatusOK, gin.H{
		"message": "All set! Time to build something awesome",
	})
}
