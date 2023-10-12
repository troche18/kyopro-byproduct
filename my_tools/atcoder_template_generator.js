// ==UserScript==
// @name         AtCoder Template Generator
// @namespace    atcoder-template-generator
// @version      1
// @description  Generate template file for AtCoder contests with .py extension and download it on button click
// @author       Your Name
// @match        https://atcoder.jp/contests/*/tasks/*
// @grant        GM_setValue
// @grant        GM_getValue
// @grant        GM_deleteValue
// ==/UserScript==

(function () {
    "use strict";

    // Constants
    const STORAGE_KEY = "atcoder-template-generator";
    const CONTEST_MODE = "contest";
    const VIRTUAL_CONTEST_MODE = "virtual-contest";
    const DOWNLOAD_BUTTON_TEXT = "Download Template";
    const ADD_BUTTON_TEXT = "Add";
    const REMOVE_BUTTON_TEXT = "Remove";
    const TEMPLATE_FILE_EXTENSION = ".py";

    // Variables
    let currentMode = CONTEST_MODE;
    let currentContestName = "";
    let currentVirtualContestName = "";
    let virtualContestNames = [];

    // DOM elements
    const wrapper = document.createElement("div"); // 他の要素をまとめるdiv要素を追加で作成
    const downloadButton = document.createElement("button");
    const modeSelect = document.createElement("select");
    const contestNameInput = document.createElement("input");
    const virtualContestNameSelect = document.createElement("select");
    const addVirtualContestNameButton = document.createElement("button");
    const removeVirtualContestNameButton = document.createElement("button");
    const virtualContestNameInput = document.createElement("input");

    // Set styles for DOM elements
    wrapper.style.display = "flex"; //　div要素にdisplay:flexを設定することで横並びに
    downloadButton.style.position = "fixed";
    downloadButton.style.bottom = "80px";
    downloadButton.style.right = "20px";
    downloadButton.style.fontSize = "16px";
    downloadButton.style.padding = "10px";
    downloadButton.style.border = "none";
    downloadButton.style.backgroundColor = "#4CAF50";
    downloadButton.style.color = "white";
    downloadButton.style.cursor = "pointer";

    modeSelect.style.width = "200px";
    modeSelect.style.right = "10px";

    contestNameInput.style.width = "300px";
    contestNameInput.style.right = "10px";

    virtualContestNameSelect.style.width = "200px";
    virtualContestNameSelect.style.right = "10px";

    addVirtualContestNameButton.style.right = "10px";

    removeVirtualContestNameButton.style.right = "10px";

    virtualContestNameInput.style.width = "200px";
    virtualContestNameInput.style.right = "10px";

    // Set default values for DOM elements
    downloadButton.innerText = DOWNLOAD_BUTTON_TEXT;

    modeSelect.add(new Option("Contest", CONTEST_MODE));
    modeSelect.add(new Option("Virtual Contest", VIRTUAL_CONTEST_MODE));

    contestNameInput.type = "text";
    contestNameInput.placeholder = "Contest Name";

    virtualContestNameInput.type = "text";
    virtualContestNameInput.placeholder = "Virtual Contest Name";

    addVirtualContestNameButton.innerText = ADD_BUTTON_TEXT;

    removeVirtualContestNameButton.innerText = REMOVE_BUTTON_TEXT;

    // Add event listeners to DOM elements
    downloadButton.addEventListener("click", () => {
        generateTemplateFile(getTaskName());
    });

    modeSelect.addEventListener("change", () => {
        switchMode(modeSelect.value);
    });

    addVirtualContestNameButton.addEventListener("click", () => {
        addVirtualContestName(virtualContestNameInput.value);
    });

    removeVirtualContestNameButton.addEventListener("click", () => {
        removeVirtualContestName(virtualContestNameSelect.value);
    });

    // Load saved settings
    loadSettings();

    // Add DOM elements to the page
    const buttonParent = document.querySelector(".col-sm-12");
    // buttonParentの下にwrapper(div要素)を追加
    buttonParent.appendChild(wrapper); // div要素以下に付け替え
    wrapper.appendChild(downloadButton);
    wrapper.appendChild(modeSelect);
    wrapper.appendChild(contestNameInput);
    wrapper.appendChild(virtualContestNameInput);
    wrapper.appendChild(addVirtualContestNameButton);
    wrapper.appendChild(virtualContestNameSelect);
    wrapper.appendChild(removeVirtualContestNameButton);
    for (let i = 0; i < virtualContestNames.length; i++) {
        const option = new Option(
            virtualContestNames[i],
            virtualContestNames[i]
        );
        virtualContestNameSelect.add(option);
    }

    // Helper functions
    function switchMode(mode) {
        currentMode = mode;
        saveSettings();
        if (mode === CONTEST_MODE) {
            contestNameInput.style.display = "block";
            virtualContestNameInput.style.display = "none";
            virtualContestNameSelect.style.display = "none";
            addVirtualContestNameButton.style.display = "none";
            removeVirtualContestNameButton.style.display = "none";
        } else if (mode === VIRTUAL_CONTEST_MODE) {
            contestNameInput.style.display = "none";
            virtualContestNameInput.style.display = "inline-block";
            virtualContestNameSelect.style.display = "inline-block";
            addVirtualContestNameButton.style.display = "inline-block";
            removeVirtualContestNameButton.style.display = "inline-block";
        }
    }

    function addVirtualContestName(name) {
        if (name !== "" && !virtualContestNames.includes(name)) {
            virtualContestNames.push(name);
            const option = new Option(name, name);
            virtualContestNameSelect.add(option);
            saveSettings();
        }
    }

    function removeVirtualContestName(name) {
        if (virtualContestNames.includes(name)) {
            virtualContestNames.splice(virtualContestNames.indexOf(name), 1);
            for (let i = 0; i < virtualContestNameSelect.options.length; i++) {
                if (virtualContestNameSelect.options[i].value === name) {
                    virtualContestNameSelect.remove(i);
                    break;
                }
            }
            saveSettings();
        }
    }

    function generateTemplateFile(taskName) {
        let fileName;
        if (currentMode === CONTEST_MODE) {
            fileName = taskName;
        } else if (currentMode === VIRTUAL_CONTEST_MODE) {
            const selectedName = virtualContestNameSelect.value;
            const now = new Date();
            const timezoneOffset = 9 * 60;
            const currentDate = new Date(
                now.getTime() + timezoneOffset * 60 * 1000
            )
                .toISOString()
                .slice(0, 10)
                .replace(/-/g, "");
            fileName = selectedName + "_" + currentDate;
            const _index = virtualContestNames.indexOf(selectedName);
            const pop = virtualContestNames.splice(_index, 1)[0];
            virtualContestNames.unshift(pop);
            saveSettings();
        }
        const templateContent =
            '# import pypyjit;pypyjit.set_param("max_unroll_recursion=-1")\n# from bisect import *\n# from collections import *\n# from heapq import *\n# from itertools import *\n# from sortedcontainers import *\n# from math import gcd, lcm\n# from datetime import *\n# from decimal import *  # PyPyだと遅い\n# from string import ascii_lowercase,ascii_uppercase\n# import numpy as np\n# from atcoder.dsu import *\n# from atcoder.segtree import *\n# from sortedcontainers import *\n# from random import *\nimport sys\nimport os\n\nis_test = os.getenv("ATCODER", 0)\n# sys.setrecursionlimit(10**6) # PyPyは呪文を付ける\nINF = 1 << 61\nMOD = 998244353\n# MOD = 10**9 + 7\nFile = sys.stdin\n\n\ndef input():\n    return File.readline()[:-1]\n\n\n# ///////////////////////////////////////////////////////////////////////////\n\n\nN = int(input())\n';
        const blob = new Blob([templateContent], {
            type: "text/x-python",
        });
        const url = URL.createObjectURL(blob);
        const a = document.createElement("a");
        a.href = url;
        a.download = fileName + TEMPLATE_FILE_EXTENSION;
        document.body.appendChild(a);
        a.click();
        setTimeout(() => {
            document.body.removeChild(a);
            URL.revokeObjectURL(url);
        }, 0);
    }

    function getTaskName() {
        const url = window.location.href;
        const parts = url.split("/");
        return parts[parts.length - 1];
    }

    function saveSettings() {
        GM_setValue(
            STORAGE_KEY,
            JSON.stringify({
                mode: currentMode,
                contestName: currentContestName,
                virtualContestName: currentVirtualContestName,
                virtualContestNames: virtualContestNames,
            })
        );
    }

    function loadSettings() {
        const settingsStr = GM_getValue(STORAGE_KEY);
        if (settingsStr !== undefined) {
            const settings = JSON.parse(settingsStr);
            currentMode = settings.mode;
            currentContestName = settings.contestName;
            currentVirtualContestName = settings.virtualContestName;
            virtualContestNames = settings.virtualContestNames;
        }
        switchMode(currentMode);
        contestNameInput.value = currentContestName;
        virtualContestNameInput.value = currentVirtualContestName;
    }

    function clearSettings() {
        GM_deleteValue(STORAGE_KEY);
    }

    // returns [{input, output, anchor}]
    function getTestCases() {
        const selectors = [
            ["#task-statement p+pre.literal-block", ".section"], // utpc2011_1
            ["#task-statement pre.source-code-for-copy", ".part"],
            ["#task-statement .lang>*:nth-child(1) .div-btn-copy+pre", ".part"],
            ["#task-statement .div-btn-copy+pre", ".part"],
            ["#task-statement>.part pre.linenums", ".part"], // abc003_4
            ["#task-statement>.part:not(.io-style)>h3+section>pre", ".part"],
            ["#task-statement pre", ".part"],
        ];

        for (const [selector, closestSelector] of selectors) {
            const e = [...$selectAll(selector)].filter((e) => {
                if ($(e).closest(".io-style").length) return false;
                return true;
            });
            if (e.length == 0) continue;
            const testcases = [];
            for (let i = 0; i < e.length; i += 2) {
                const container =
                    e[i].closest(closestSelector) || e[i].parentElement;
                testcases.push({
                    input: (e[i] || {}).textContent,
                    output: (e[i + 1] || {}).textContent,
                    anchor: container.querySelector("h3"),
                });
            }
            return testcases;
        }

        return [];
    }
})();
