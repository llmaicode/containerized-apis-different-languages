package com.example.todo;


import org.springframework.boot.SpringApplication;
import org.springframework.boot.autoconfigure.SpringBootApplication;

import org.springframework.web.bind.annotation.*;

import org.springframework.stereotype.Service;
import org.springframework.stereotype.Repository;

import org.springframework.data.jpa.repository.JpaRepository;

import jakarta.persistence.*;

import jakarta.validation.Valid;
import jakarta.validation.constraints.NotBlank;

import org.springframework.http.HttpStatus;
import org.springframework.web.server.ResponseStatusException;

import java.util.List;


@SpringBootApplication
public class TodoApplication {

    public static void main(String[] args) {

        SpringApplication.run(
            TodoApplication.class,
            args
        );

    }


    // ============================
    // ENTITY
    // ============================

    @Entity
    static class Todo {

        @Id
        @GeneratedValue(
            strategy = GenerationType.IDENTITY
        )
        public Long id;


        @Column(nullable = false)
        public String title;


        public String description;


        public boolean completed = false;

    }


    // ============================
    // DTO
    // ============================


    static class TodoRequest {


        @NotBlank
        public String title;


        public String description;


        public Boolean completed;


    }


    // ============================
    // REPOSITORY
    // ============================


    @Repository
    interface TodoRepository
            extends JpaRepository<Todo,Long>{}


    // ============================
    // SERVICE
    // ============================


    @Service
    static class TodoService {


        private final TodoRepository repository;


        TodoService(
            TodoRepository repository
        ){

            this.repository =
                repository;

        }


        public Todo create(
            TodoRequest request
        ){

            Todo todo =
                new Todo();

            todo.title =
                request.title;

            todo.description =
                request.description;

            return repository.save(todo);

        }


        public List<Todo> findAll(){

            return repository.findAll();

        }


        public Todo findOne(
            Long id
        ){

            return repository.findById(id)

                .orElseThrow(
                    () ->
                    new ResponseStatusException(
                        HttpStatus.NOT_FOUND,
                        "Todo not found"
                    )
                );

        }


        public Todo update(
            Long id,
            TodoRequest request
        ){

            Todo todo =
                findOne(id);


            if(request.title != null)
                todo.title =
                    request.title;


            if(request.description != null)
                todo.description =
                    request.description;


            if(request.completed != null)
                todo.completed =
                    request.completed;


            return repository.save(todo);

        }


        public String delete(
            Long id
        ){

            Todo todo =
                findOne(id);

            repository.delete(todo);


            return "Todo deleted successfully";

        }


    }


    // ============================
    // CONTROLLER
    // ============================


    @RestController
    @RequestMapping("/todos")
    static class TodoController {


        private final TodoService service;


        TodoController(
            TodoService service
        ){

            this.service =
                service;

        }


        @PostMapping
        public Todo create(
            @Valid
            @RequestBody TodoRequest request
        ){

            return service.create(request);

        }


        @GetMapping
        public List<Todo> findAll(){

            return service.findAll();

        }


        @GetMapping("/{id}")
        public Todo findOne(
            @PathVariable Long id
        ){

            return service.findOne(id);

        }


        @PutMapping("/{id}")
        public Todo update(
            @PathVariable Long id,
            @RequestBody TodoRequest request
        ){

            return service.update(
                id,
                request
            );

        }


        @DeleteMapping("/{id}")
        public String delete(
            @PathVariable Long id
        ){

            return service.delete(id);

        }


    }

}