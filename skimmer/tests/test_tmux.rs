use skimmer::tmux::TmuxOptions;

#[test]
fn test_tmux_option_default() {
    let opt = "center,50%".to_string();
    let opts: TmuxOptions = TmuxOptions::from(&opt);
    assert_eq!(opts.width, "50%");
    assert_eq!(opts.height, "50%");
    assert_eq!(opts.x, "C");
    assert_eq!(opts.y, "C");
}

// CENTER

#[test]
fn test_tmux_option_center_one_percent() {
    let opt = "center,20%".to_string();
    let opts: TmuxOptions = TmuxOptions::from(&opt);
    assert_eq!(opts.width, "20%");
    assert_eq!(opts.height, "20%");
    assert_eq!(opts.x, "C");
    assert_eq!(opts.y, "C");
}

#[test]
fn test_tmux_option_center_both_percent() {
    let opt = "center,50%,30%".to_string();
    let opts: TmuxOptions = TmuxOptions::from(&opt);
    assert_eq!(opts.width, "50%");
    assert_eq!(opts.height, "30%");
    assert_eq!(opts.x, "C");
    assert_eq!(opts.y, "C");
}

#[test]
fn test_tmux_option_center_one_fixed() {
    let opt = "center,50".to_string();
    let opts: TmuxOptions = TmuxOptions::from(&opt);
    assert_eq!(opts.width, "50");
    assert_eq!(opts.height, "50");
    assert_eq!(opts.x, "C");
    assert_eq!(opts.y, "C");
}

#[test]
fn test_tmux_option_center_both_fixed() {
    let opt = "center,10,80".to_string();
    let opts: TmuxOptions = TmuxOptions::from(&opt);
    assert_eq!(opts.width, "10");
    assert_eq!(opts.height, "80");
    assert_eq!(opts.x, "C");
    assert_eq!(opts.y, "C");
}

#[test]
fn test_tmux_option_center_mixed() {
    let opt = "center,10,80%".to_string();
    let opts: TmuxOptions = TmuxOptions::from(&opt);
    assert_eq!(opts.width, "10");
    assert_eq!(opts.height, "80%");
    assert_eq!(opts.x, "C");
    assert_eq!(opts.y, "C");
}

// TOP

#[test]
fn test_tmux_option_top_one_percent() {
    let opt = "top,20%".to_string();
    let opts: TmuxOptions = TmuxOptions::from(&opt);
    assert_eq!(opts.width, "100%");
    assert_eq!(opts.height, "20%");
    assert_eq!(opts.x, "C");
    assert_eq!(opts.y, "0%");
}

#[test]
fn test_tmux_option_top_both_percent() {
    let opt = "top,50%,30%".to_string();
    let opts: TmuxOptions = TmuxOptions::from(&opt);
    assert_eq!(opts.width, "30%");
    assert_eq!(opts.height, "50%");
    assert_eq!(opts.x, "C");
    assert_eq!(opts.y, "0%");
}

#[test]
fn test_tmux_option_top_one_fixed() {
    let opt = "top,50".to_string();
    let opts: TmuxOptions = TmuxOptions::from(&opt);
    assert_eq!(opts.width, "100%");
    assert_eq!(opts.height, "50");
    assert_eq!(opts.x, "C");
    assert_eq!(opts.y, "0%");
}

#[test]
fn test_tmux_option_top_both_fixed() {
    let opt = "top,10,80".to_string();
    let opts: TmuxOptions = TmuxOptions::from(&opt);
    assert_eq!(opts.width, "80");
    assert_eq!(opts.height, "10");
    assert_eq!(opts.x, "C");
    assert_eq!(opts.y, "0%");
}

#[test]
fn test_tmux_option_top_mixed() {
    let opt = "top,10,80%".to_string();
    let opts: TmuxOptions = TmuxOptions::from(&opt);
    assert_eq!(opts.width, "80%");
    assert_eq!(opts.height, "10");
    assert_eq!(opts.x, "C");
    assert_eq!(opts.y, "0%");
}

// BOTTOM

#[test]
fn test_tmux_option_bottom_one_percent() {
    let opt = "bottom,20%".to_string();
    let opts: TmuxOptions = TmuxOptions::from(&opt);
    assert_eq!(opts.width, "100%");
    assert_eq!(opts.height, "20%");
    assert_eq!(opts.x, "C");
    assert_eq!(opts.y, "100%");
}

#[test]
fn test_tmux_option_bottom_both_percent() {
    let opt = "bottom,50%,30%".to_string();
    let opts: TmuxOptions = TmuxOptions::from(&opt);
    assert_eq!(opts.width, "30%");
    assert_eq!(opts.height, "50%");
    assert_eq!(opts.x, "C");
    assert_eq!(opts.y, "100%");
}

#[test]
fn test_tmux_option_bottom_one_fixed() {
    let opt = "bottom,50".to_string();
    let opts: TmuxOptions = TmuxOptions::from(&opt);
    assert_eq!(opts.width, "100%");
    assert_eq!(opts.height, "50");
    assert_eq!(opts.x, "C");
    assert_eq!(opts.y, "100%");
}

#[test]
fn test_tmux_option_bottom_both_fixed() {
    let opt = "bottom,10,80".to_string();
    let opts: TmuxOptions = TmuxOptions::from(&opt);
    assert_eq!(opts.width, "80");
    assert_eq!(opts.height, "10");
    assert_eq!(opts.x, "C");
    assert_eq!(opts.y, "100%");
}

#[test]
fn test_tmux_option_bottom_mixed() {
    let opt = "bottom,10,80%".to_string();
    let opts: TmuxOptions = TmuxOptions::from(&opt);
    assert_eq!(opts.width, "80%");
    assert_eq!(opts.height, "10");
    assert_eq!(opts.x, "C");
    assert_eq!(opts.y, "100%");
}

// LEFT

#[test]
fn test_tmux_option_left_one_percent() {
    let opt = "left,20%".to_string();
    let opts: TmuxOptions = TmuxOptions::from(&opt);
    assert_eq!(opts.height, "100%");
    assert_eq!(opts.width, "20%");
    assert_eq!(opts.y, "C");
    assert_eq!(opts.x, "0%");
}

#[test]
fn test_tmux_option_left_both_percent() {
    let opt = "left,50%,30%".to_string();
    let opts: TmuxOptions = TmuxOptions::from(&opt);
    assert_eq!(opts.height, "30%");
    assert_eq!(opts.width, "50%");
    assert_eq!(opts.y, "C");
    assert_eq!(opts.x, "0%");
}

#[test]
fn test_tmux_option_left_one_fixed() {
    let opt = "left,50".to_string();
    let opts: TmuxOptions = TmuxOptions::from(&opt);
    assert_eq!(opts.height, "100%");
    assert_eq!(opts.width, "50");
    assert_eq!(opts.y, "C");
    assert_eq!(opts.x, "0%");
}

#[test]
fn test_tmux_option_left_both_fixed() {
    let opt = "left,10,80".to_string();
    let opts: TmuxOptions = TmuxOptions::from(&opt);
    assert_eq!(opts.height, "80");
    assert_eq!(opts.width, "10");
    assert_eq!(opts.y, "C");
    assert_eq!(opts.x, "0%");
}

#[test]
fn test_tmux_option_left_mixed() {
    let opt = "left,10,80%".to_string();
    let opts: TmuxOptions = TmuxOptions::from(&opt);
    assert_eq!(opts.height, "80%");
    assert_eq!(opts.width, "10");
    assert_eq!(opts.y, "C");
    assert_eq!(opts.x, "0%");
}

// BOTTOM

#[test]
fn test_tmux_option_right_one_percent() {
    let opt = "right,20%".to_string();
    let opts: TmuxOptions = TmuxOptions::from(&opt);
    assert_eq!(opts.height, "100%");
    assert_eq!(opts.width, "20%");
    assert_eq!(opts.y, "C");
    assert_eq!(opts.x, "100%");
}

#[test]
fn test_tmux_option_right_both_percent() {
    let opt = "right,50%,30%".to_string();
    let opts: TmuxOptions = TmuxOptions::from(&opt);
    assert_eq!(opts.height, "30%");
    assert_eq!(opts.width, "50%");
    assert_eq!(opts.y, "C");
    assert_eq!(opts.x, "100%");
}

#[test]
fn test_tmux_option_right_one_fixed() {
    let opt = "right,50".to_string();
    let opts: TmuxOptions = TmuxOptions::from(&opt);
    assert_eq!(opts.height, "100%");
    assert_eq!(opts.width, "50");
    assert_eq!(opts.y, "C");
    assert_eq!(opts.x, "100%");
}

#[test]
fn test_tmux_option_right_both_fixed() {
    let opt = "right,10,80".to_string();
    let opts: TmuxOptions = TmuxOptions::from(&opt);
    assert_eq!(opts.height, "80");
    assert_eq!(opts.width, "10");
    assert_eq!(opts.y, "C");
    assert_eq!(opts.x, "100%");
}

#[test]
fn test_tmux_option_right_mixed() {
    let opt = "right,10,80%".to_string();
    let opts: TmuxOptions = TmuxOptions::from(&opt);
    assert_eq!(opts.height, "80%");
    assert_eq!(opts.width, "10");
    assert_eq!(opts.y, "C");
    assert_eq!(opts.x, "100%");
}
