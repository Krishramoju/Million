pub fn analyze_soil(moisture: f32, ph: f32) -> String {
    if moisture < 30.0 {
        "Moisture low: Start irrigation.".into()
    } else if ph < 5.5 {
        "Soil too acidic: Add lime.".into()
    } else {
        "Soil conditions optimal.".into()
    }
}

pub fn suggest_crop(weather: &str) -> String {
    match weather {
        "hot" => "Suggest planting millet or sorghum.".into(),
        "rainy" => "Ideal for rice or sugarcane.".into(),
        _ => "Crop choice unclear, gather more data.".into(),
    }
}
