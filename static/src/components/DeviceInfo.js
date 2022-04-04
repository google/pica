import { LitElement, html, css } from "lit";

import "./Orientation.js";

export class DeviceInfo extends LitElement {
  static styles = css`
    :host {
      display: flex;
      flex-direction: column;
      padding: 12px;
      color: white;
    }

    h1 {
      margin: 0 auto;
      font-weight: 700;
    }

    h2 {
      font-weight: 500;
      margin: 12px;
    }

    .secondary {
      color: #4f46e5;
    }

    .orientation {
      margin-top: 10px;
      display: flex;
      flex-direction: column;
      margin-bottom: 2px;
    }

    .orientation > pika-orientation {
      width: 200px;
      height: 200px;
      margin: auto;
    }

    label {
      display: flex;
      flex-direction: column;
    }

    .neighbors {
      display: flex;
      flex-direction: column;
      padding: 0;
      margin: 0;
      list-style: none;
      overflow-y: scroll;
    }

    .neighbors > li {
      display: flex;
      flex-direction: row;
      align-items: center;
    }

    .neighbors > li > table {
      color: inherit;
    }

    .neighbors > li > table td {
      padding: 0 6px;
    }

    .neighbors > li > pika-orientation {
      width: 100px;
      height: 100px;
    }
  `;

  static properties = {
    device: {},
  };

  constructor() {
    super();
    this.device = null;
  }

  setAzimuth(event) {
    this.device.azimuth = Number(event.target.value);
    this.update();
  }

  setElevation(event) {
    this.device.elevation = Number(event.target.value);
    this.update();
  }

  notifyChange() {
    this.dispatchEvent(new CustomEvent("orientation-change"));
  }

  render() {
    return html`
      <h1>Pica</h1>
      <h2>Selected Device</h2>
      ${this.device == null
        ? html`<span>No Device Selected</span>`
        : html`
            <span>Mac Address: 0x${this.device.mac_address.toString(16)}</span>
            <span
              >X:
              <span style="color: red">${this.device.position.x}</span></span
            >
            <span
              >Y:
              <span style="color: blue">${this.device.position.y}</span></span
            >
            <span
              >Z:
              <span style="color: green">${this.device.position.z}</span></span
            >
            <div class="orientation">
              <span class="center">Device Orientation</span>
              <pika-orientation
                azimuth=${this.device.azimuth}
                elevation=${this.device.elevation}
              ></pika-orientation>
              <label>
                <span>Azimuth (${this.device.azimuth})</span>
                <input
                  type="range"
                  min="-180"
                  max="180"
                  value=${this.device.azimuth}
                  @input=${this.setAzimuth}
                  @change=${this.notifyChange}
                />
              </label>
              <label>
                <span>Elevation (${this.device.elevation})</span>
                <input
                  type="range"
                  min="-90"
                  max="90"
                  value=${this.device.elevation}
                  @input=${this.setElevation}
                  @change=${this.notifyChange}
                />
              </label>
            </div>
            <h2>Neighbors</h2>
            <ul class="neighbors">
              ${this.device.neighbors.map(
                ({ mac_address, azimuth, elevation }) => html`
                  <li>
                    <pika-orientation
                      azimuth="${azimuth}"
                      elevation="${elevation}"
                    ></pika-orientation>
                    <table>
                      <tr>
                        <td>Mac</td>
                        <td>0x${mac_address.toString(16)}</td>
                      </tr>
                      <tr>
                        <td>Azimuth</td>
                        <td>${azimuth}</td>
                      </tr>
                      <tr>
                        <td>Elevation</td>
                        <td>${elevation}</td>
                      </tr>
                    </table>
                  </li>
                `
              )}
            </ul>
          `}
    `;
  }
}
customElements.define("pika-device-info", DeviceInfo);