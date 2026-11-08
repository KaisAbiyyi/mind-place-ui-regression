# Shared Visual & LOD Rendering Specification (Phase 3)
## Mind Place Knowledge Graph ("Hive Mind")

This specification defines the exact mathematical, visual, and level-of-detail (LOD) rules that MUST be obeyed identically by all three benchmark backends:
1. **Native GPUI** (Direct3D 11 via Lyon & GPUI Scene)
2. **WebView2 Canvas 2D** (HTML5 Canvas 2D Context)
3. **WebView2 WebGL2** (Hardware Shaders & Vertex Buffers)

---

## 1. Coordinate Space & Camera Transformation

Given graph-space coordinates $(x, y)$, canvas width $W$, canvas height $H$, camera pan $(cx, cy)$, and camera zoom $s$:

$$\text{screen}_x = \frac{W}{2} + (x - cx) \cdot s$$
$$\text{screen}_y = \frac{H}{2} + (y - cy) \cdot s$$

---

## 2. Node Geometry & Hierarchy

### Effective Screen-Space Radius ($r_{\text{screen}}$)
For a node $n$ with base radius $n.r$:
- $\text{offset}_{\text{sel}} = 3.5$ if selected, else $0$
- $\text{offset}_{\text{hover}} = 2.0$ if hovered and not selected, else $0$
- Base scale factor: $k_{\text{scale}} = \max(0.68, s)$
- Normal state:
  $$r_{\text{screen}} = (n.r + \text{offset}_{\text{sel}} + \text{offset}_{\text{hover}}) \cdot k_{\text{scale}}$$
- Dimmed state (when search filter active and node does not match):
  $$r_{\text{screen, dimmed}} = r_{\text{screen}} \cdot 0.75$$

### Node Visual Layers (Back-to-Front)
1. **Focus Ring (Selected or Hovered only):**
   - Radius: $r_{\text{ring}} = r_{\text{screen}} + 6.0 \cdot \max(0.70, s)$
   - Border width: $1.2\text{ px}$
   - Dash pattern: `[3, 3]`
   - Color: `canvas_edge_active` (#FFFFFF) for selected, `text_muted` (#71717A) for hovered.
2. **Node Circle Body:**
   - Radius: $r_{\text{screen}}$
   - Fill: `canvas_node_fill` (#FFFFFF) if selected or root ("mind"); `canvas_node_secondary` (#27272A) if 1-hop branch; `canvas_node_stroke` (#52525B) for standard nodes; dimmed to #27272A if unmatched.
   - Border width: $2.2\text{ px}$ if selected; $1.4\text{ px}$ otherwise.
   - Border color: `canvas_node_stroke` if selected; `canvas_node_fill` otherwise.
3. **Inner Core Dot (Standard hollow nodes only):**
   - Active when: not selected, not 1-hop branch, and not root "mind".
   - Radius: $\max(2.0, r_{\text{screen}} \cdot 0.35)$
   - Fill: `canvas_node_secondary` (#27272A).

---

## 3. Edge Geometry & Styling

For an edge connecting node $A$ at screen point $P = (P_x, P_y)$ and node $B$ at screen point $Q = (Q_x, Q_y)$:

### Quadratic Bézier Control Point ($M$)
The control point $M = (M_x, M_y)$ is offset perpendicularly to the chord $(P, Q)$ by exactly $6\%$ of chord length:
$$M_x = \frac{P_x + Q_x}{2} + (Q_y - P_y) \cdot 0.06$$
$$M_y = \frac{P_y + Q_y}{2} - (Q_x - P_x) \cdot 0.06$$

### Active vs Inactive Styling
- An edge is **Active** if and only if both $A$ and $B$ are in the active relationship set:
  $$\text{activeSet} = \{\text{selected}\} \cup \text{branches}(\text{selected})$$
- **Inactive / Standard Edge:**
  - Thickness: $w = 0.85 \cdot \max(0.75, s)$
  - Dash array: `[3, 5]`
  - Color: `canvas_edge` (#3F3F46); if dimmed: `canvas_edge_dim` (#27272A)
- **Active / Highlighted Edge:**
  - Thickness: $w = 1.50 \cdot \max(0.75, s)$
  - Dash array: Solid (no dash)
  - Color: `canvas_edge_active` (#FFFFFF)

### Data Pulse Particle
- Rendered if edge is active, trace animation is enabled, and edge is not dimmed.
- Travels along $B(t) = (1-t)^2 P + 2(1-t)t M + t^2 Q$ where:
  $$t = ((\text{time} \cdot 0.00018 + (|A.\text{id}| \cdot |B.\text{id}| \cdot 0.031)) \bmod 1.0 + 1.0) \bmod 1.0$$
- Particle radius: $2.5\text{ px}$ solid circle, fill `canvas_node_fill` (#FFFFFF).

---

## 4. Shared Label Visibility Contract

### Criteria for Rendering a Node Label
A label for node $n$ is rendered **if and only if** all of the following conditions are met:
1. **Viewport Intersection:** The node screen coordinate $(P_x, P_y)$ satisfies:
   $$-80 \le P_x \le W + 80 \quad \text{and} \quad -80 \le P_y \le H + 80$$
2. **Filter Match:** Node is not culled by an active category filter.
3. **Threshold Condition:**
   - **Full-Fidelity Mode:**
     Render label if:
     $$(n.r \ge 9.0) \lor \text{isSelected} \lor \text{isHovered} \lor (s > 1.35 \land r_{\text{screen}} \ge 6.0)$$
   - **Optimized LOD Mode:**
     Render label if:
     $$\text{isSelected} \lor \text{isHovered} \lor (r_{\text{screen}} \ge 12.0) \lor (s > 1.80 \land r_{\text{screen}} \ge 8.0)$$

### Label Typography & Layout
- Font: Inter / sans-serif, weight 700 if selected, weight 500 otherwise.
- Size: $\text{clamp}(11.0 \cdot s, 10.0, 13.0)\text{ px}$.
- Alignment: Centered horizontally at $P_x$, top baseline at:
  $$\text{baseline}_y = P_y + r_{\text{screen}} + 6.0 \cdot \max(0.70, s)$$
- Color: `canvas_label` (#FFFFFF) if selected; `canvas_label_muted` (#A1A1AA) if normal; `text_muted` (#52525B) if dimmed.

---

## 5. Viewport Culling & LOD Specifications

### Full-Fidelity Mode (Baseline Parity)
- **Curves:** 100% of visible edges are rendered as true quadratic Bézier curves with exact thickness and dash patterns.
- **Labels:** Rendered according to the Full-Fidelity label contract.
- **No geometric degradation:** Off-screen geometry is culled from paint lists, but every on-screen primitive is fully rendered.

### Optimized LOD Mode (Scalability Architecture)
- **Spatial Culling:** Quad-tree or spatial grid culls nodes and edges outside screen viewport plus 100px buffer.
- **Edge Curve Simplification:**
  - If $s \ge 0.75$: Render true quadratic Bézier curve.
  - If $s < 0.75$: Render straight line segment between $(P_x, P_y)$ and $(Q_x, Q_y)$. The visual deflection ($< 1.0\text{ px}$) is imperceptible at low scale.
- **Dashed Array Simplification:**
  - If $s < 0.50$: Render dashed edges as solid thin lines ($w = 0.6\text{ px}$), avoiding excessive sub-pixel dash tessellation.
- **Label LOD:** Follows Optimized LOD label contract.

---

## 6. Telemetry & Counter Contract

Every frame rendered MUST count and report:
- `nodes_total`: Total nodes in graph dataset.
- `nodes_visible`: Nodes passing layer filter.
- `nodes_rendered`: Nodes actually drawn on screen.
- `edges_total`: Total edges in graph dataset.
- `edges_rendered`: Edges actually drawn on screen.
- `labels_rendered`: Count of text labels shaped and painted.
- `primitives_culled`: Count of off-screen primitives culled by viewport check.
- `cpu_update_ms`: Time spent in simulation, culling, and camera math.
- `cpu_draw_ms`: Time spent in geometry tessellation, canvas drawing, or GPU submission.
- `frame_interval_ms`: Inter-frame cadence delta.
