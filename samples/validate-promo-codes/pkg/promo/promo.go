package promo

import (
	"encoding/json"
	"math/rand"
	"strings"
	"time"

	"github.com/google/uuid"
	"github.com/spinframework/spin-go-sdk/v3/kv"
)

type PromoCode struct {
	Code      string    `json:"code"`
	ValidFrom time.Time `json:"validFrom"`
	ValidTo   time.Time `json:"validTo"`
	Discount  float32   `json:"discount"`
	Used      bool      `json:"used"`
}

type ApplyResult struct {
	Code     string  `json:"code"`
	Valid    bool    `json:"isValid"`
	Reason   string  `json:"reason,omitempty"`
	Discount float32 `json:"discount"`
}

type ValidationResult struct {
	Code     string  `json:"code"`
	Valid    bool    `json:"isValid"`
	Reason   string  `json:"reason"`
	Discount float32 `json:"discount"`
}

func invalid(input string, reason string) *ValidationResult {
	return &ValidationResult{
		Code:     input,
		Valid:    false,
		Reason:   reason,
		Discount: 0,
	}
}

func Apply(input string) (*ApplyResult, error) {
	v, err := Validate(input)
	if err != nil {
		return nil, err
	}
	if !v.Valid {
		return &ApplyResult{
			Code:     input,
			Valid:    false,
			Reason:   v.Reason,
			Discount: 0,
		}, nil
	}
	store, err := kv.Open("default")
	if err != nil {
		return nil, err
	}
	data, err := store.Get(strings.ToLower(input))
	if err != nil {
		return nil, err
	}
	var code PromoCode
	err = json.Unmarshal(data, &code)
	if err != nil {
		return nil, err
	}
	code.Used = true
	newData, err := json.Marshal(code)
	if err != nil {
		return nil, err
	}
	err = store.Set(strings.ToLower(code.Code), newData)
	if err != nil {
		return nil, err
	}
	return &ApplyResult{
		Code:     input,
		Valid:    true,
		Reason:   "",
		Discount: code.Discount,
	}, nil
}

func Validate(input string) (*ValidationResult, error) {
	store, err := kv.Open("default")
	if err != nil {
		return nil, err
	}
	exists, err := store.Exists(strings.ToLower(input))
	if err != nil {
		return nil, err
	}
	if !exists {
		return invalid(input, "Invalid code presented"), nil
	}

	bytes, err := store.Get(strings.ToLower(input))
	if err != nil {
		return nil, err
	}
	var code PromoCode
	err = json.Unmarshal(bytes, &code)
	if err != nil {
		return nil, err
	}
	if code.Used {
		return invalid(input, "Code has already been used"), nil
	}
	now := time.Now()

	if now.After(code.ValidFrom) && now.Before(code.ValidTo) {
		return &ValidationResult{
			Code:     input,
			Valid:    true,
			Discount: code.Discount,
		}, nil
	}
	return invalid(input, "Code could not be applied at this time"), nil
}

func NewPromoCode(validFrom, validTo time.Time, discount float32) PromoCode {
	return PromoCode{
		Code:      uuid.NewString(),
		ValidFrom: validFrom.Truncate(24 * time.Hour),
		ValidTo:   validTo.Truncate(24 * time.Hour),
		Discount:  discount,
	}
}

func GenerateSamplePromoCodes() []PromoCode {
	var promoCodes []PromoCode
	now := time.Now().Truncate(24 * time.Hour)

	for i := 0; i < 50; i++ {
		validFrom := now.AddDate(0, 0, rand.Intn(30))
		validTo := validFrom.AddDate(0, 6, 0)
		discount := rand.Float32() * 0.35

		promoCode := NewPromoCode(validFrom, validTo, discount)
		promoCodes = append(promoCodes, promoCode)
	}

	return promoCodes
}

func StoreCodes(codes []PromoCode) error {
	store, err := kv.Open("default")
	if err != nil {
		return err
	}

	for _, item := range codes {
		code, err := json.Marshal(item)
		if err != nil {
			return err
		}
		err = store.Set(item.Code, code)
		if err != nil {
			return err
		}
	}
	return nil
}
