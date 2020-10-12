/*
 * Exemplar-Rust
 *
 * An exemplar rust repo with an openapi spec
 *
 * API version: 0.0.1
 * Contact: decline@umass.edu
 * Generated by: OpenAPI Generator (https://openapi-generator.tech)
 */

package main

import (
	"log"
	"net/http"

	openapi "github.com/Galactus-Player/gexemplar/go"
)

func main() {
	log.Printf("Server started")

	DefaultApiService := openapi.NewDefaultApiService()
	DefaultApiController := openapi.NewDefaultApiController(DefaultApiService)

	router := openapi.NewRouter(DefaultApiController)

	log.Fatal(http.ListenAndServe(":8080", router))
}