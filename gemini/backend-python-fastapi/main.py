from typing import Optional, List

from fastapi import FastAPI, Depends, HTTPException
from pydantic import BaseModel

from sqlalchemy import create_engine, Column, Integer, String, Boolean
from sqlalchemy.orm import declarative_base, sessionmaker, Session


# -----------------------------
# Database Configuration
# -----------------------------

DATABASE_URL = (
    "postgresql+psycopg2://postgres:password@localhost:5432/todo_db"
)

engine = create_engine(
    DATABASE_URL,
    pool_pre_ping=True
)

SessionLocal = sessionmaker(
    autocommit=False,
    autoflush=False,
    bind=engine
)

Base = declarative_base()


# -----------------------------
# Database Model
# -----------------------------

class Todo(Base):
    __tablename__ = "todos"

    id = Column(Integer, primary_key=True, index=True)
    title = Column(String(255), nullable=False)
    description = Column(String(500), nullable=True)
    completed = Column(Boolean, default=False)


# Create tables
Base.metadata.create_all(bind=engine)


# -----------------------------
# Pydantic Schemas
# -----------------------------

class TodoCreate(BaseModel):
    title: str
    description: Optional[str] = None


class TodoUpdate(BaseModel):
    title: Optional[str] = None
    description: Optional[str] = None
    completed: Optional[bool] = None


class TodoResponse(BaseModel):
    id: int
    title: str
    description: Optional[str]
    completed: bool

    class Config:
        from_attributes = True


# -----------------------------
# FastAPI App
# -----------------------------

app = FastAPI(
    title="Todo API",
    version="1.0"
)


# -----------------------------
# Database Dependency
# -----------------------------

def get_db():
    db = SessionLocal()
    try:
        yield db
    finally:
        db.close()


# -----------------------------
# CRUD Routes
# -----------------------------

# CREATE
@app.post(
    "/todos",
    response_model=TodoResponse
)
def create_todo(
    todo: TodoCreate,
    db: Session = Depends(get_db)
):
    new_todo = Todo(
        title=todo.title,
        description=todo.description
    )

    db.add(new_todo)
    db.commit()
    db.refresh(new_todo)

    return new_todo


# READ ALL
@app.get(
    "/todos",
    response_model=List[TodoResponse]
)
def get_todos(
    db: Session = Depends(get_db)
):
    return db.query(Todo).all()


# READ ONE
@app.get(
    "/todos/{todo_id}",
    response_model=TodoResponse
)
def get_todo(
    todo_id: int,
    db: Session = Depends(get_db)
):
    todo = (
        db.query(Todo)
        .filter(Todo.id == todo_id)
        .first()
    )

    if not todo:
        raise HTTPException(
            status_code=404,
            detail="Todo not found"
        )

    return todo


# UPDATE
@app.put(
    "/todos/{todo_id}",
    response_model=TodoResponse
)
def update_todo(
    todo_id: int,
    todo_update: TodoUpdate,
    db: Session = Depends(get_db)
):
    todo = (
        db.query(Todo)
        .filter(Todo.id == todo_id)
        .first()
    )

    if not todo:
        raise HTTPException(
            status_code=404,
            detail="Todo not found"
        )

    if todo_update.title is not None:
        todo.title = todo_update.title

    if todo_update.description is not None:
        todo.description = todo_update.description

    if todo_update.completed is not None:
        todo.completed = todo_update.completed

    db.commit()
    db.refresh(todo)

    return todo


# DELETE
@app.delete(
    "/todos/{todo_id}"
)
def delete_todo(
    todo_id: int,
    db: Session = Depends(get_db)
):
    todo = (
        db.query(Todo)
        .filter(Todo.id == todo_id)
        .first()
    )

    if not todo:
        raise HTTPException(
            status_code=404,
            detail="Todo not found"
        )

    db.delete(todo)
    db.commit()

    return {
        "message": "Todo deleted successfully"
    }