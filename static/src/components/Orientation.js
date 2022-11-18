// Copyright 2022 Google LLC
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     https://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

import { LitElement, html, css, unsafeCSS } from "lit";

import { PROJECTION } from "./Map.js";

export class Orientation extends LitElement {
  static styles = css`
    :host {
      transform-style: preserve-3d;
      transform: ${unsafeCSS(PROJECTION.toString())};
      perspective-origin: center;
      display: flex;
      position: relative;
    }

    .axis {
      position: absolute;
      left: 50%;
      width: 3px;
      height: 50%;
      transform-origin: bottom center;
      background: currentColor;
    }

    .z {
      color: blue;
    }

    .x {
      color: green;
    }

    .y {
      color: red;
    }

    .circle {
      position: absolute;
      left: 17.5%;
      top: 17.5%;
      height: 65%;
      width: 65%;
      opacity: 0.3;
    }

    .value {
      color: yellow;
    }

    .arrow::after {
      content: "";
      display: block;
      border-left: 2px solid currentColor;
      border-top: 2px solid currentColor;
      width: 10px;
      height: 10px;
      transform: rotate(45deg) translate(0, -2px);
      transform-origin: top left;
    }
  `;

  static properties = {
    yaw: { type: Number },
    pitch: { type: Number },
    roll: { type: Number },
  };

  constructor() {
    super();
    this.yaw = 0;
    this.pitch = 0;
    this.roll = 0;
  }

  render() {
    const a = (0.5 / Math.PI) * 360 * 4;

    const { yaw, pitch } = this;

    return html`
      <div class="axis arrow x" style="transform: rotateZ(-180deg)"></div>
      <div class="axis arrow y" style="transform: rotateZ(-90deg)"></div>
      <div class="axis arrow z" style="transform: rotateX(-90deg)"></div>
      <svg
        viewBox="${-a / 2} ${-a / 2} ${a} ${a}"
        class="circle"
        style="transform: rotateZ(90deg) rotateX(${yaw < 0 ? 0 : 180}deg)"
      >
        <circle
          r="${a / 4}"
          stroke-width="${a / 2}"
          stroke-dasharray="${Math.abs(yaw)} 360"
          stroke="blue"
          fill="none"
        />
      </svg>
      <svg
        viewBox="${-a / 2} ${-a / 2} ${a} ${a}"
        class="circle"
        style="transform: rotateZ(${90 - yaw}deg) rotateX(${Math.sign(pitch) *
        90}deg)"
      >
        <circle
          r="${a / 4}"
          stroke-width="${a / 2}"
          stroke-dasharray="${Math.abs(pitch)} 360"
          stroke="red"
          fill="none"
        />
      </svg>
      <div
        class="axis value"
        style="transform: rotateZ(${180 -
        yaw}deg) rotateX(${-pitch}deg) rotateY(90deg)"
      ></div>
      <div
        class="axis value"
        style="transform: rotateZ(${180 - yaw}deg) rotateX(${-pitch}deg)"
      ></div>
    `;
  }
}
customElements.define("pika-orientation", Orientation);
