import express from "express"
import routers from "./router/routers";
import { Kafka, Partitioners } from "kafkajs";
const kafka= new Kafka({
    clientId:"Axness",
    brokers:["localhost:9092"]
})

export const producer = kafka.producer(
    {
        createPartitioner:Partitioners.LegacyPartitioner
    }
)
function setup (){
    producer.connect().then(()=>{
        console.log("kafka connected!")
    })
}

setup()
const app = express();
const user:string[]= []

app.use("/api/v1",routers)

app.listen(3000)
