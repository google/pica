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

    .x {
        color: blue;
    }

    .y {
        color: green;
    }

    .z {
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
        content: '';
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
    azimuth: { type: Number },
    elevation: { type: Number },
  };

  constructor() {
      super();
      this.azimuth = 0;
      this.elevation = 0;
  }

  render() {
    const a = (0.5 / Math.PI) * 360 * 4;

    const { azimuth, elevation } = this;

    return html`
      <div class="axis arrow x" style="transform: rotateZ(-180deg)"></div>
      <div class="axis arrow y" style="transform: rotateZ(-90deg)"></div>
      <div class="axis arrow z" style="transform: rotateX(-90deg)"></div>
      <svg viewBox="${-a / 2} ${-a / 2} ${a} ${a}" class="circle" style="transform: rotateZ(90deg) rotateX(${azimuth > 0 ? 0 : 180}deg)">
        <circle
          r="${a / 4}"
          stroke-width="${a / 2}"
          stroke-dasharray="${Math.abs(azimuth)} 360"
          stroke="blue"
          fill="none"
        />
      </svg>
      <svg viewBox="${-a / 2} ${-a / 2} ${a} ${a}" class="circle" style="transform: rotateZ(${90 + azimuth}deg) rotateX(${Math.sign(elevation) * 90}deg)">
        <circle
          r="${a / 4}"
          stroke-width="${a / 2}"
          stroke-dasharray="${Math.abs(elevation)} 360"
          stroke="red"
          fill="none"
        />
      </svg>
      <div class="axis value" style="transform: rotateZ(${180 + azimuth}deg) rotateX(${-elevation}deg) rotateY(90deg)"></div>
      <div class="axis value" style="transform: rotateZ(${180 + azimuth}deg) rotateX(${-elevation}deg)"></div>
    `;
  }
}
customElements.define("pika-orientation", Orientation);
