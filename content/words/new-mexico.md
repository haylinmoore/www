---
title: AI Recolors New Mexico State Flag
description: using a neural network to find the perfect color scheme
date: 2018-10-18
tags:
    - machine-learning
    - javascript
---

I'm bad at design, but good at programming. Which means I am very good at design. If your first response to that was "That makes no sense, you just said you are bad" then you are kinda correct. I really liked the results of this, but that is likely due to my personal bias.

### Backstory
I was looking at state flags one day and came to the realization that the New Mexico flag one of the only good looking flags. I only had one issue with it, the coloring. I don’t know what about it I did not like, but I just knew I did not particularly like the color scheme. After a bit of time, I had an idea. Since I don’t know what I like, why not have a computer do it for me. 

### How?
It is super simple. First, it generates 2 random colors, then renders it onto the user's screen. Then the user has four choices, Love it, Like it, Meh, and Hate it. It then feeds the Neural Network a value, 1 for Love it all the way to 0 for Hate it. It uses this in the future to find the perfect color scheme. The user then continues to do this for as long as they wish. Once they think it has been long enough it gets tired, then click the dandy “Generate” button. That runs 200 thousand rounds of generating random colors, sending it to the trained and personalized neural network, and save it to an array. After all the simulations finish, it then finds the color scheme with the higher score and displays it to the user.

<svg xmlns="http://www.w3.org/2000/svg" width="400" height="300" xmlns:xlink="http://www.w3.org/1999/xlink"
    viewBox="0 0 1200 800" style="max-width: 100%;">
    <rect class="nm_flag" fill="#fff700" width="1200" height="800" />
    <g transform="translate(600,400)" stroke="#bf0a30" class="nm_star">
        <path id="lin"
            d="M157.344,38.2812H-157.344M191.375,12.75H-191.375M191.375-12.75H-191.375M157.344-38.2812H-157.344"
            stroke-width="17" stroke-linecap="round" />
        <use transform="rotate(90)" xlink:href="#lin" />
        <circle class="nm_flag" fill="#ffd700" r="64.3125" stroke-width="10.625" />
    </g>
</svg>
<br>
<button onclick="rank(1)">Love It</button>
<button onclick="rank(0.4)">Like it</button>
<button onclick="rank(0.1)">Meh</button>
<button onclick="rank(0)">Hate It</button>
<button onclick="bestFlag(100000)">Generate Good Looking</button>
<h3 id="data"></h3>
<p>Just click your thought on the image, the more you click the better it looks!</p>

<script src='https://unpkg.com/brain.js@1.1.2/browser.min.js'></script>
<script>
    var net = new brain.NeuralNetwork();

    var data = [{
        input: [0, 0, 0, 0, 0, 0],
        output: [0]
    }, {
        input: [1, 1, 1, 1, 1, 1],
        output: [0]
    }];

    net.train(data);

    var times = 0;

    const getRandomColorValue = (maxVal) => {
        return Math.floor(Math.random() * maxVal);
    };

    function rank(value) {

        times++;

        data.push({
            input: colors,
            output: [value]
        });

        if (times === 1) {
            net.train(data);
        }

        colors = [getRandomColorValue(255) / 255, getRandomColorValue(255) / 255, getRandomColorValue(255) / 255, getRandomColorValue(255) / 255, getRandomColorValue(255) / 255, getRandomColorValue(255) / 255];

        setStarColor([colors[0], colors[1], colors[2]]);

        setFlagColor([colors[3], colors[4], colors[5]]);

        updateUser(net.run(colors));

    }

    function setStarColor(color) {
        document.getElementsByClassName("nm_star")[0].style.stroke = `rgb(${color[0]*255},${color[1]*255},${color[2]*255})`;
    }

    function updateUser(data) {
        document.getElementById("data").innerHTML = "Chance You May Like it " + (data[0] * 100).toFixed(1) + "%";
    }

    function setFlagColor(color) {
        document.getElementsByClassName("nm_flag")[0].style.fill = `rgb(${color[0]*255},${color[1]*255},${color[2]*255})`;
        document.getElementsByClassName("nm_flag")[1].style.fill = `rgb(${color[0]*255},${color[1]*255},${color[2]*255})`;
    }

    var colors = [0.74901960784, 0.03921568627, 0.18823529412, 1, 0.8431372549, 0];

    setStarColor([colors[0], colors[1], colors[2]]);

    setFlagColor([colors[3], colors[4], colors[5]]);

    updateUser(net.run(colors));


    function bestFlag(loops) {
        net.train(data);
        var results = [];
        for (let i = 0; i < loops; i++) {
            colors = [getRandomColorValue(255) / 255, getRandomColorValue(255) / 255, getRandomColorValue(255) / 255,
                getRandomColorValue(255) / 255, getRandomColorValue(255) / 255, getRandomColorValue(255) / 255
            ];
            const [score] = net.run(colors);
            results.push({
                ...colors,
                score
            });
        }

        const sortedResults = results.sort((a, b) => b.score - a.score);



        colors = sortedResults[0];

        delete colors.score;

        colors = [sortedResults[0][0], sortedResults[0][1], sortedResults[0][2], sortedResults[0][3], sortedResults[0][4], sortedResults[0][5]];

        setStarColor([colors[0], colors[1], colors[2]]);

        setFlagColor([colors[3], colors[4], colors[5]]);

        updateUser(net.run(colors));

    }
</script>