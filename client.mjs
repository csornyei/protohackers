import { Socket } from "net";
import * as readline from "readline";

const client = new Socket();

client.connect(8080, "0.0.0.0");

client.on("data", (data) => {
  console.log(data.toString());
});

client.on("close", () => {
  console.log("Connection closed");
});

client.on("error", (err) => {
  console.log(err);
});

const rl = readline.createInterface({
  input: process.stdin,
  output: process.stdout,
});

rl.on("line", (input) => {
  client.write(input);
});

process.on("SIGINT", () => {
  client.end();
  process.exit();
});
