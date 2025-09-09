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
async function setup (){
    await producer.connect()
}

setup()
const app = express();
app.use(express.json())
const user:string[]= []

app.use("/api/v1",routers)

app.listen(3000)
