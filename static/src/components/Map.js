import { LitElement, css, html, svg } from "lit";

function createIsometricMatrix() {
  const m = new DOMMatrix();

  const angle = (Math.atan(Math.SQRT2) * 180) / Math.PI;

  m.rotateAxisAngleSelf(1, 0, 0, angle);
  m.rotateAxisAngleSelf(0, 0, 1, -45);
  m.scaleSelf(0.7, 0.7, 0.7);

  return m;
}

export const PROJECTION = createIsometricMatrix();

const OFFSET_X = 100 + 40;
const OFFSET_Y = 100;

export class Map extends LitElement {
  static styles = css`
    svg {
      width: var(--map-size);
      height: min(var(--map-size), 100vh);
    }

    .dragging {
      cursor: grab;
    }
  `;

  static properties = {
    devices: {},
  };

  constructor() {
    super();
    this.devices = [];
    this.selected = null;
    this.dragging = false;
    this.changingElevation = false;
    this.onMouseUp = this.onMouseUp.bind(this);
  }

  connectedCallback() {
    super.connectedCallback();
    window.addEventListener("mouseup", this.onMouseUp);
  }

  disconnectedCallback() {
    window.removeEventListener("mouseup", this.onMouseUp);
    super.disconnectedCallback();
  }

  onMouseDown(event) {
    if (event.target.classList?.contains("handle")) {
      this.changingElevation = true;
    } else {
      const element = event.path.find((el) => el.classList?.contains("marker"));
      if (element) {
        const key = element.getAttribute("key");
        this.selected = this.devices[key];
        this.dragging = true;
      } else {
        this.selected = null;
      }
      this.dispatchEvent(
        new CustomEvent("select", { detail: { device: this.selected } })
      );
      this.update();
    }
  }

  onMouseUp() {
    this.dragging = false;
    this.changingElevation = false;
    this.update();
  }

  onMouseMove(event) {
    if (this.dragging) {
      const to = this.screenToSvg(event.clientX, event.clientY);
      this.selected.position.x = Math.max(
        Math.floor(to.x) - this.selected.position.z - OFFSET_X,
        0
      );
      this.selected.position.y = Math.max(
        Math.floor(to.y) + this.selected.position.z - OFFSET_Y,
        0
      );
      this.update();
    }
    if (this.changingElevation) {
      const distance = Math.min(event.movementY * 2, this.selected.position.z);
      this.selected.position.z -= distance;
      this.update();
    }
    if (this.dragging || this.changingElevation) {
      this.dispatchEvent(new CustomEvent("move"));
    }
  }

  screenToSvg(x, y) {
    const svg = this.renderRoot.children[0];
    const point = svg.createSVGPoint();
    point.x = x;
    point.y = y;
    return point.matrixTransform(svg.getScreenCTM().inverse());
  }

  render() {
    const m = PROJECTION;
    return html`<svg
      transform="matrix(${m.a} ${m.b} ${m.c} ${m.d} ${m.e} ${m.f})"
      @mousemove="${this.onMouseMove}"
      @mousedown="${this.onMouseDown}"
      class="${this.dragging ? "dragging" : ""}"
    >
      <defs>
        <pattern
          id="small"
          width="20"
          height="20"
          patternUnits="userSpaceOnUse"
        >
          <path
            d="M20,0 L0,0 L0,20"
            fill="none"
            stroke="var(--grid-line-color)"
            stroke-width="1"
          />
        </pattern>
        <pattern
          id="grid"
          width="100"
          height="100"
          patternUnits="userSpaceOnUse"
        >
          <rect width="100" height="100" fill="url(#small)" />
          <path
            d="M100,0 L0,0 L0,100"
            fill="none"
            stroke="var(--grid-line-color)"
            stroke-width="2"
          />
        </pattern>
      </defs>

      <rect fill="var(--grid-background-color)" width="100%" height="100%" />
      <rect fill="url(#grid)" width="100%" height="100%"></rect>

      ${this.devices.map(
        (device, i) => svg`
    <g key="${i}" transform=${`translate(${
          device.position.x + OFFSET_X + device.position.z
        } ${device.position.y + OFFSET_Y - device.position.z})`}
        class="${device == this.selected ? "selected marker" : "marker"}">
        <rect x="-40" y="-40" width="40" height="40" fill="#f44336" transform="skewY(-45)"></rect>
        <rect x="0" y="0" width="40" height="40" fill="#ffeb3b" transform="skewX(-45)"></rect>
        <rect x="0" y="-40" width="40" height="40" fill="#2196f3" transform=></rect>

        <!-- Selection outline -->
        <g stroke="var(--selection-color)" fill="var(--selection-color)" stroke-width="5">
            ${
              device == this.selected
                ? svg`
            <line x1="-40" y1="-40" x2="0" y2="-40" transform="skewY(-45)"></line>
            <line x1="-40" y1="-40" x2="-40" y2="0" transform="skewY(-45)"></line>
            <line x1="0" y1="40" x2="40" y2="40" transform="skewX(-45)"></line>
            <line x1="40" y1="40" x2="40" y2="0" transform="skewX(-45)"></line>
            <line x1="0" y1="-40" x2="40" y2="-40"></line>
            <line x1="40" y1="-40" x2="40" y2="0"></line>

            <circle cx="36" cy="-36" r="8" class="handle"></circle>
            `
                : ""
            }
        </g>

        <!-- Elevation arrow -->
        <g stroke="black" stroke-width="5">
            ${
              device.position.z == 0
                ? ""
                : svg`
            <line x1="-40" y1="0" x2="-${
              40 + device.position.z
            }" y2="0" transform="skewY(-45)" stroke-dasharray="8" />
            <path d="M-${40 + device.position.z},-10 L-${
                    40 + device.position.z
                  },10" transform="skewY(-45)" />
            <path d="M-${40 + device.position.z},-10 L-${
                    40 + device.position.z
                  },10" transform="skewY(-45) skewX(45)">
                `
            }
        </g>
    </g>
    `
      )}
    </svg>`;
  }
}
customElements.define("pika-map", Map);
