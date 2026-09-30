package main

import (
	"encoding/json"
	"net/http"

	"github.com/akamai-developers/akamai-functions-samples/samples/validate-promo-codes/pkg/promo"
	spinhttp "github.com/spinframework/spin-go-sdk/v3/http"
)

func init() {
	router := http.NewServeMux()

	router.HandleFunc("POST /apply/{code}", applyPromocode)
	router.HandleFunc("POST /validate/{code}", validatePromoCode)
	router.HandleFunc("POST /seed-promocodes", seedPromoCodes)
	spinhttp.Handle(func(w http.ResponseWriter, r *http.Request) {
		router.ServeHTTP(w, r)
	})
}

func applyPromocode(w http.ResponseWriter, r *http.Request) {
	input := r.PathValue("code")
	if len(input) == 0 {
		http.Error(w, "Bad Request", 400)
		return
	}
	res, err := promo.Apply(input)
	if err != nil {
		http.Error(w, "Internal Server Error", 500)
		return
	}
	if !res.Valid {
		w.WriteHeader(400)
		h := w.Header()
		h.Set("content-type", "application/json")
		encoder := json.NewEncoder(w)
		encoder.SetIndent("", "  ")
		encoder.Encode(res)
		return
	}

	w.WriteHeader(200)
	h := w.Header()
	h.Set("content-type", "application/json")
	encoder := json.NewEncoder(w)
	encoder.SetIndent("", "  ")
	encoder.Encode(res)
}

func validatePromoCode(w http.ResponseWriter, r *http.Request) {
	input := r.PathValue("code")
	if len(input) == 0 {
		http.Error(w, "Bad Request", 400)
		return
	}
	res, err := promo.Validate(input)
	if err != nil {
		http.Error(w, "Internal Server Error", 500)
		return
	}
	if !res.Valid {
		w.WriteHeader(400)
		h := w.Header()
		h.Set("content-type", "application/json")
		encoder := json.NewEncoder(w)
		encoder.SetIndent("", "  ")
		encoder.Encode(res)
		return
	}

	w.WriteHeader(200)
	h := w.Header()
	h.Set("content-type", "application/json")
	encoder := json.NewEncoder(w)
	encoder.SetIndent("", "  ")
	encoder.Encode(res)
}

func seedPromoCodes(w http.ResponseWriter, r *http.Request) {
	codes := promo.GenerateSamplePromoCodes()
	promo.StoreCodes(codes)
	w.WriteHeader(201)
	h := w.Header()
	h.Set("content-type", "application/json")
	encoder := json.NewEncoder(w)
	encoder.SetIndent("", "  ")
	encoder.Encode(codes)
}
func main() {}
