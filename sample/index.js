import * as wasm from "wasm-pinyin";

const hans = "但是，近些年，商店已经不再是人们唯一的选择了。通过网络来买东西正在成为一种常见的消费方式。不少人甚至几个月都不去商店一次。";
const result = wasm.pinyin(hans);
console.log(result);
