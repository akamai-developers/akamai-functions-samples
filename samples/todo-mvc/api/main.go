package main

import (
	"encoding/json"
	"fmt"
	"net/http"

	"github.com/api/internal/repository"
	spinhttp "github.com/spinframework/spin-go-sdk/v3/http"
)

func init() {
	router := http.NewServeMux()
	router.HandleFunc("GET /api/todos", getAllTodos)
	router.HandleFunc("POST /api/todos", addNewTodo)
	router.HandleFunc("DELETE /api/todos/{id}", deleteTodoById)
	router.HandleFunc("PUT /api/todos/{id}", updateTodoById)
	router.HandleFunc("POST /api/todos/{id}", toggleTodoById)
	router.HandleFunc("DELETE /api/todos", deleteCompletedTodos)
	spinhttp.Handle(func(w http.ResponseWriter, r *http.Request) {
		router.ServeHTTP(w, r)
	})
}

type CreateAndUpdateModel struct {
	Content string `json:"content"`
}

func getAllTodos(w http.ResponseWriter, r *http.Request) {
	todos, err := repository.GetAllTodos()
	if err != nil {
		fmt.Printf("GET /api/todos: %v", err)
		http.Error(w, "Internal Server Error", 500)
		return
	}
	w.Header().Add("content-type", "application/json")
	enc := json.NewEncoder(w)
	err = enc.Encode(todos)
	if err != nil {
		http.Error(w, "Internal Server Error", 500)
		return
	}
}

func addNewTodo(w http.ResponseWriter, r *http.Request) {
	dec := json.NewDecoder(r.Body)
	var payload CreateAndUpdateModel
	err := dec.Decode(&payload)
	if err != nil {
		http.Error(w, "Bad Request", 400)
		return
	}
	t, err := repository.AddTodo(payload.Content)
	if err != nil {
		fmt.Printf("POST /api/todos: %v", err)
		http.Error(w, "Internal Server Error", 500)
		return
	}
	w.Header().Add("content-type", "application/json")
	w.WriteHeader(201)
	enc := json.NewEncoder(w)
	err = enc.Encode(t)
	if err != nil {
		http.Error(w, "Internal Server Error", 500)
		return
	}

}

func deleteTodoById(w http.ResponseWriter, r *http.Request) {
	id := r.PathValue("id")
	deleted, err := repository.DeleteTodoById(id)
	if err != nil {
		fmt.Printf("DELETE /api/todos/%s: %v", id, err)
		http.Error(w, "Internal Server Error", 500)
		return
	}
	if !deleted {
		w.WriteHeader(404)
		w.Write([]byte("Task Not found"))
		return
	}
	w.WriteHeader(204)
}

func updateTodoById(w http.ResponseWriter, r *http.Request) {
	id := r.PathValue("id")
	dec := json.NewDecoder(r.Body)
	var payload CreateAndUpdateModel
	err := dec.Decode(&payload)
	if err != nil {
		http.Error(w, "Bad Request", 400)
		return
	}

	updated, err := repository.UpdateTodo(id, payload.Content)
	if err != nil {
		fmt.Printf("PUT /api/todos/%s: %v", id, err)
		http.Error(w, "Internal Server Error", 500)
		return
	}
	if updated == nil {
		w.Write([]byte("Todo Not Found"))
		w.WriteHeader(404)
		return
	}
	w.Header().Add("content-type", "application/json")
	w.WriteHeader(200)
	enc := json.NewEncoder(w)
	err = enc.Encode(updated)
	if err != nil {
		http.Error(w, "Internal Server Error", 500)
		return
	}
}

func toggleTodoById(w http.ResponseWriter, r *http.Request) {
	id := r.PathValue("id")

	toggled, err := repository.ToggleTodoById(id)
	if err != nil {
		fmt.Printf("POST /api/todos/%s: %v", id, err)
		http.Error(w, "Internal Server Error", 500)
		return
	}
	if toggled == nil {
		w.Write([]byte("Todo Not Found"))
		w.WriteHeader(404)
		return
	}
	w.Header().Add("content-type", "application/json")
	w.WriteHeader(200)
	enc := json.NewEncoder(w)
	err = enc.Encode(toggled)
	if err != nil {
		http.Error(w, "Internal Server Error", 500)
		return
	}

}

func deleteCompletedTodos(w http.ResponseWriter, r *http.Request) {
	todos, err := repository.DeleteAllCompletedTodos()
	if err != nil {
		fmt.Printf("DELETE /api/todos/completed: %v", err)
		http.Error(w, "Internal Server Error", 500)
		return
	}
	w.Header().Add("content-type", "application/json")
	enc := json.NewEncoder(w)
	err = enc.Encode(todos)
	if err != nil {
		http.Error(w, "Internal Server Error", 500)
		return
	}
}

func main() {}
