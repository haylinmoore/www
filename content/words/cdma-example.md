---
title: CDMA
description: an interactive example of CDMA modulation
date: 2024-02-22
tags:
  - radio
---

## CDMA Interactive Demo

<style>
  td {
    width: 1.5rem;
  }
</style>
<div style="font-family: monospace">
  <br>
  <input type="range" id="walsh_depth" name="walsh" min="1" max="4" value="1" />
  <label for="walsh">Senders: <span id="senders_count">2</span></label>
  <br>
  First the desired sender count must be chosen so that sender codes can be calculated.

  <h4>Walsh Table</h4>
  A <a href="https://en.wikipedia.org/wiki/Walsh_matrix">Walsh matrix</a> is used to generate orthogonal codes 
  <table id="walsh" border="1">
    <tbody align="right"></tbody>
  </table>
  <h4>Per Sender Code</h4>
  Each sender's orthogonal is taken from the relevant row on the Walsh table
  <pre id="senders_list">
  </pre>
  <h4>Encoded Message</h4>
  Each sender can send one trit (-1, 0, 1) at a time. 
  The first input corresponds to sender 0's data point, the second one is sender 1, etc
  <div id="inputs"></div>

  <br>
  <input id="calculate" type="button" value="Calculate" />
  <div id="calculation" style="display: none">
    <h4>Per Sender Transmission</h4>
    The sender's data (input i) multiplied by the senders code
    <pre id="transmission"></pre>
    <h4>Received Transmission</h4>
    This is the sum of the transmissions, as the waves from each sender constructively and destructively collide
    <pre id="received"></pre>
    <h4>Decoded Transmission</h4>
    By multiplying the received transmission by the sender's code, the original data can be recovered
    <pre id="decoded"></pre>
  </div>
</div>

<script>
const arrayLength = (n) => new Array(n).fill(undefined);

/*
  Making a Walsh table
  W1 = [1]
  W2i = [ Wi Wi ]
        [ Wi WiC]
*/
const walsh_round = (matrix) => {
  const len = matrix.length
  let out = arrayLength(len*2).map(v=>arrayLength(len*2))
  
  matrix.forEach((line, row) => {
    line.forEach((v, col) => {
      out[row][col] = v;
      out[row][col + len] = v;
      out[row + len][col] = v;
      out[row + len][col + len] = v * -1;
    })
  })
  return out;
}

const walsh = (n) => arrayLength(n).reduce((inc)=>walsh_round(inc), [[1]])

let state = {
  depth: 1,
  senders: 2,
  codes: [],
}

const tritSameLength = (v) => String(v).padStart(2, " ")
const swapTableBody = (table, array2D) => table.querySelector('tbody').innerHTML = array2D.map(row => `<tr>${row.map(cell => `<td>${tritSameLength(cell)}</td>`).join('')}</tr>`).join('');

const updateWalsh = (v) => {
  state.depth = v; 
  state.senders = Math.pow(2, state.depth)

  // Generate the walsh matrix
  state.codes = walsh(state.depth);

  // Display the walsh matrix
  swapTableBody(document.getElementById("walsh"), state.codes)
  document.getElementById("senders_count").innerHTML = state.senders;
  document.getElementById("senders_list").innerHTML = state.codes.map((v, i)=>`Sender ${String(i).padStart(state.senders > 9? 2: 1, " ")}: [${v.map(tritSameLength).join(" ")} ]`).join("\n")
  document.getElementById("inputs").innerHTML = state.codes.map(()=>`<input type="number" min="-1" max="1" value="0">`).join(" ")
}

const walshDepth = document.getElementById("walsh_depth").addEventListener("change", (elm)=>{
  updateWalsh(Number(elm.target.value))
})

updateWalsh(1)

document.getElementById("calculate").addEventListener("click", ()=>{
  // Get the sender's data from the inputs
  const code = [...document.getElementById("inputs").children].map(v=>Number(v.value))

  // Multiply the sender's data by their code to get their transmission
  const transmit = state.codes.map((chip, sender_index)=>chip.map(v=>v * code[sender_index]))

  

  // Simulate the transmission by summing the transmissions from each sender
  const recv = arrayLength(state.senders)
    .map(
      (_, i) => transmit.reduce((acc, sender)=>acc+sender[i], 0)
    )

  // Recover the data by multiplying the received transmission by the sender's code
  const recovered_data = arrayLength(state.senders)
    .map((_, senderIndex)=> {
      const senderCode = state.codes[senderIndex];
      const multi = senderCode.map((v, j)=>v*recv[j]);
      // Sum each sender's matrix into a single value
      const val = multi.reduce((a, v)=>a+v, 0);

      return `Sender ${senderIndex}: [${recv.map(tritSameLength).join(" ")} ] * [${senderCode.map(tritSameLength).join(" ")} ] = [${multi.map(tritSameLength).join(" ")} ]\n\t- Sum: ${val}, Value: ${val/state.senders}\n`
  })

  // Display the results
  document.getElementById("received").innerHTML = `[ ${recv.join(" ")} ]`
  document.getElementById("transmission").innerHTML = transmit.map((v, i)=>`Sender ${String(i).padStart(state.senders > 9? 2: 1, " ")}: [${v.map(tritSameLength).join(" ")} ]`).join("\n")
  document.getElementById("decoded").innerHTML = recovered_data.join("\n");
  document.getElementById("calculation").style.display = "block";
})

</script>

## Why

Recently, in my wireless networking class we were covering CDMA, and I found it difficult to understand how the encoding and decoding process worked.
I made this interactive example to help me understand it better.
I felt that having something interactive could have better helped me understand the concept, and I hope it helps you too.
