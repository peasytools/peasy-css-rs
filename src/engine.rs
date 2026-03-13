/// Generate a CSS linear gradient.
pub fn linear_gradient(colors: &[&str], direction: &str) -> String {
    format!("background: linear-gradient({}, {});", direction, colors.join(", "))
}

/// Generate a CSS radial gradient.
pub fn radial_gradient(colors: &[&str], shape: &str) -> String {
    format!("background: radial-gradient({}, {});", shape, colors.join(", "))
}

/// Generate a CSS box shadow.
pub fn box_shadow(x: i32, y: i32, blur: i32, spread: i32, color: &str) -> String {
    format!("box-shadow: {}px {}px {}px {}px {};", x, y, blur, spread, color)
}

/// Generate a CSS text shadow.
pub fn text_shadow(x: i32, y: i32, blur: i32, color: &str) -> String {
    format!("text-shadow: {}px {}px {}px {};", x, y, blur, color)
}

/// Generate CSS border-radius.
pub fn border_radius(radii: &[u32]) -> String {
    let vals: Vec<String> = radii.iter().map(|r| format!("{}px", r)).collect();
    format!("border-radius: {};", vals.join(" "))
}

/// Generate CSS flexbox layout.
pub fn flexbox(direction: &str, justify: &str, align: &str, wrap: &str, gap: Option<&str>) -> String {
    let mut lines = vec![
        "display: flex".to_string(),
        format!("flex-direction: {}", direction),
        format!("justify-content: {}", justify),
        format!("align-items: {}", align),
        format!("flex-wrap: {}", wrap),
    ];
    if let Some(g) = gap { lines.push(format!("gap: {}", g)); }
    lines.join(";\n") + ";"
}

/// Generate CSS grid layout.
pub fn grid(columns: u32, gap: &str, min_width: Option<&str>) -> String {
    let col = match min_width {
        Some(mw) => format!("repeat(auto-fit, minmax({}, 1fr))", mw),
        None => format!("repeat({}, 1fr)", columns),
    };
    format!("display: grid;\ngrid-template-columns: {};\ngap: {};", col, gap)
}

/// Generate CSS transform.
pub fn transform(rotate: Option<f64>, scale: Option<f64>, tx: Option<f64>, ty: Option<f64>) -> String {
    let mut parts = Vec::new();
    if let Some(r) = rotate { parts.push(format!("rotate({}deg)", r)); }
    if let Some(s) = scale { parts.push(format!("scale({})", s)); }
    if let Some(x) = tx { parts.push(format!("translateX({}px)", x)); }
    if let Some(y) = ty { parts.push(format!("translateY({}px)", y)); }
    format!("transform: {};", parts.join(" "))
}

/// Generate CSS transition.
pub fn transition(property: &str, duration: &str, easing: &str, delay: Option<&str>) -> String {
    let mut val = format!("{} {} {}", property, duration, easing);
    if let Some(d) = delay { val.push(' '); val.push_str(d); }
    format!("transition: {};", val)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_linear_gradient() {
        let css = linear_gradient(&["#ff0000", "#0000ff"], "to right");
        assert!(css.contains("linear-gradient"));
        assert!(css.contains("#ff0000"));
    }

    #[test]
    fn test_box_shadow() {
        let css = box_shadow(0, 4, 6, 0, "rgba(0,0,0,0.1)");
        assert_eq!(css, "box-shadow: 0px 4px 6px 0px rgba(0,0,0,0.1);");
    }

    #[test]
    fn test_border_radius() {
        assert_eq!(border_radius(&[8]), "border-radius: 8px;");
        assert_eq!(border_radius(&[8, 16]), "border-radius: 8px 16px;");
    }

    #[test]
    fn test_grid() {
        let css = grid(3, "1rem", None);
        assert!(css.contains("repeat(3, 1fr)"));
    }
}
