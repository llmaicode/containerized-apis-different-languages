import { NestFactory } from '@nestjs/core';
import {
  Module,
  Injectable,
  Controller,
  Get,
  Post,
  Put,
  Delete,
  Body,
  Param,
  NotFoundException,
  ValidationPipe,
} from '@nestjs/common';

import {
  TypeOrmModule,
  InjectRepository,
} from '@nestjs/typeorm';

import {
  Entity,
  Column,
  PrimaryGeneratedColumn,
  Repository,
} from 'typeorm';

import {
  SwaggerModule,
  DocumentBuilder,
} from '@nestjs/swagger';

import {
  IsString,
  IsOptional,
  IsBoolean,
} from 'class-validator';

import {
  ConfigModule,
} from '@nestjs/config';


// =============================
// DATABASE ENTITY
// =============================

@Entity()
class Todo {

  @PrimaryGeneratedColumn()
  id:number;


  @Column()
  title:string;


  @Column({
    nullable:true
  })
  description:string;


  @Column({
    default:false
  })
  completed:boolean;

}


// =============================
// DTOs
// =============================

class CreateTodoDto {

  @IsString()
  title:string;


  @IsOptional()
  @IsString()
  description?:string;

}


class UpdateTodoDto {

  @IsOptional()
  @IsString()
  title?:string;


  @IsOptional()
  @IsString()
  description?:string;


  @IsOptional()
  @IsBoolean()
  completed?:boolean;

}


// =============================
// SERVICE
// =============================

@Injectable()
class TodoService {


constructor(
  @InjectRepository(Todo)
  private repo:Repository<Todo>
){}



create(dto:CreateTodoDto){

  const todo =
    this.repo.create(dto);

  return this.repo.save(todo);

}


findAll(){

  return this.repo.find();

}


async findOne(id:number){

  const todo =
    await this.repo.findOne({
      where:{
        id
      }
    });


  if(!todo){

    throw new NotFoundException(
      "Todo not found"
    );

  }


  return todo;

}


async update(
 id:number,
 dto:UpdateTodoDto
){

 const todo =
   await this.findOne(id);


 Object.assign(
   todo,
   dto
 );


 return this.repo.save(todo);

}


async remove(id:number){

 const todo =
   await this.findOne(id);


 await this.repo.remove(todo);


 return {
   message:"Todo deleted successfully"
 };

}

}


// =============================
// CONTROLLER
// =============================

@Controller('todos')
class TodoController {


constructor(
 private service:TodoService
){}




@Post()
create(
 @Body() dto:CreateTodoDto
){

 return this.service.create(dto);

}


@Get()
findAll(){

 return this.service.findAll();

}


@Get(':id')
findOne(
 @Param('id') id:string
){

 return this.service.findOne(
   Number(id)
 );

}


@Put(':id')
update(
 @Param('id') id:string,
 @Body() dto:UpdateTodoDto
){

 return this.service.update(
   Number(id),
   dto
 );

}


@Delete(':id')
remove(
 @Param('id') id:string
){

 return this.service.remove(
   Number(id)
 );

}

}


// =============================
// APP MODULE
// =============================

@Module({

imports:[

 ConfigModule.forRoot(),


 TypeOrmModule.forRoot({

  type:'postgres',

  host: process.env.DB_HOST || 'localhost',

  port: Number(process.env.DB_PORT) || 5432,

  username: process.env.DB_USERNAME || 'postgres',

  password: process.env.DB_PASSWORD || 'password',

  database: process.env.DB_DATABASE || 'todo_db',

  entities:[
    Todo
  ],

  synchronize:true

 }),


 TypeOrmModule.forFeature([
   Todo
 ])

],


controllers:[
 TodoController
],


providers:[
 TodoService
]

})


class AppModule {}


// =============================
// BOOTSTRAP
// =============================

async function bootstrap(){


const app =
 await NestFactory.create(
   AppModule
 );


app.useGlobalPipes(
 new ValidationPipe({
   whitelist:true,
   transform:true
 })
);


const swaggerConfig =
new DocumentBuilder()

.setTitle(
 'Todo API'
)

.setDescription(
 'Single file NestJS Todo CRUD API'
)

.setVersion(
 '1.0'
)

.build();


const swaggerDocument =
SwaggerModule.createDocument(
 app,
 swaggerConfig
);


SwaggerModule.setup(
 'docs',
 app,
 swaggerDocument
);


await app.listen(
 process.env.PORT || 3000
);


console.log(
 'API running: http://localhost:3000'
);

console.log(
 'Swagger: http://localhost:3000/docs'
);


}


bootstrap();