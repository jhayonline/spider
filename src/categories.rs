use once_cell::sync::Lazy;
use std::collections::HashMap;

// Jiji category to Phoenix Mall category mapping
pub static CATEGORY_MAP: Lazy<HashMap<&'static str, &'static str>> = Lazy::new(|| {
    HashMap::from([
        // Mobile Devices
        ("mobile-phones", "Smartphones"),
        ("tablets", "Tablets"),
        ("smart-watches", "Smartwatches"),
        // Computing & Electronics
        ("computers-and-laptops", "Laptops"),
        ("computer-monitors", "Monitors"),
        ("computer-hardware", "Computer Components"),
        ("computer-accessories", "Computer Accessories"),
        ("tv-dvd-equipment", "Televisions"),
        ("audio-and-music-equipment", "Audio Equipment"),
        ("headphones", "Headphones"),
        ("cameras", "Cameras"),
        // Games
        ("video-games-and-consoles", "Video Games"),
        ("videogames", "Video Games"),
        // Fashion
        ("mens-fashion", "Men's Fashion"),
        ("womens-fashion", "Women's Fashion"),
        ("baby-kids-fashion", "Kids' Fashion"),
        // Home & Living
        ("furniture", "Furniture"),
        ("home-and-office", "Home Appliances"),
        // Mobility
        ("cars", "Cars"),
        // Real Estate
        ("real-estate", "Real Estate"),
        // Beauty
        ("beauty", "Beauty & Personal Care"),
        // Services
        ("services", "Services"),
        // Pets
        ("pets-and-animals", "Pets & Animals"),
    ])
});

// Phoenix Mall category IDs (from your database)
#[allow(dead_code)]
pub static PHOENIX_CATEGORY_IDS: Lazy<HashMap<&'static str, &'static str>> = Lazy::new(|| {
    HashMap::from([
        ("Smartphones", "00000000-0000-0000-0000-000000000013"),
        ("Tablets", "00000000-0000-0000-0000-000000000014"),
        ("Smartwatches", "00000000-0000-0000-0000-000000000016"),
        ("Laptops", "00000000-0000-0000-0000-000000000017"),
        ("Monitors", "00000000-0000-0000-0000-000000000019"),
        (
            "Computer Components",
            "00000000-0000-0000-0000-000000000020",
        ),
        (
            "Computer Accessories",
            "00000000-0000-0000-0000-000000000021",
        ),
        ("Televisions", "00000000-0000-0000-0000-000000000023"),
        ("Audio Equipment", "00000000-0000-0000-0000-000000000024"),
        ("Headphones", "00000000-0000-0000-0000-000000000025"),
        ("Cameras", "00000000-0000-0000-0000-000000000026"),
        ("Video Games", "00000000-0000-0000-0000-000000000027"),
        ("Men's Fashion", "00000000-0000-0000-0000-000000000036"),
        ("Women's Fashion", "00000000-0000-0000-0000-000000000037"),
        ("Kids' Fashion", "00000000-0000-0000-0000-000000000038"),
        ("Furniture", "00000000-0000-0000-0000-000000000030"),
        ("Home Appliances", "00000000-0000-0000-0000-000000000032"),
        ("Cars", "00000000-0000-0000-0000-000000000001"),
        ("Real Estate", "00000000-0000-0000-0000-000000000002"),
        (
            "Beauty & Personal Care",
            "99999999-9999-9999-9999-999999999999",
        ),
        ("Services", "bbbbbbbb-bbbb-bbbb-bbbb-bbbbbbbbbbbb"),
        ("Pets & Animals", "dddddddd-dddd-dddd-dddd-dddddddddddd"),
    ])
});

pub fn get_phoenix_category(jiji_category: &str) -> Option<&'static str> {
    CATEGORY_MAP.get(jiji_category).copied()
}

#[allow(dead_code)]
pub fn get_phoenix_category_id(jiji_category: &str) -> Option<&'static str> {
    let category_name = CATEGORY_MAP.get(jiji_category)?;
    PHOENIX_CATEGORY_IDS.get(category_name).copied()
}
