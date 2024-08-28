use proj4rs::proj::Proj;

#[test]
fn test_projection() {
    // EPSG:3067 - TM35FIN(E,N) -- Finland
    let from = Proj::from_proj_string(
        "+proj=utm +zone=35 +ellps=GRS80 +towgs84=0,0,0,0,0,0,0 +units=m +no_defs +type=crs",
    )
    .unwrap();

    // EPSG:4326 - WGS84
    let to = Proj::from_proj_string("+proj=longlat +datum=WGS84 +no_defs +type=crs")
    .unwrap();
    
    /*  N=7369564.333, E=427997.035 */
    let epsg3067_northern = 7369564.333;
    let epsg3067_eastern = 427997.035;


    let mut point_3d = (epsg3067_eastern, epsg3067_northern, 0.0);
    proj4rs::transform::transform(&from, &to, &mut point_3d).unwrap();

    // Note that angular unit is radians, not degrees
    let (longitude, latitude,_height) = point_3d;

    // Output in longitude, latitude
    println!("LatLng: {},{}", latitude.to_degrees(), longitude.to_degrees());

    // Projection validated here:
    // https://epsg.io/transform#s_srs=3067&t_srs=4326&ops=1149&x=427997.0350000&y=7369564.3330000
    assert!(((latitude.to_degrees() * 1E6).round() / 1E6) == 66.437124, "projection match with lat");
    assert!(((longitude.to_degrees() * 1E6).round() / 1E6) == 25.385742, "projection match with lon");

    // EPSG:3067 E 427997.035 -> EPSG:4326 longitude 25°23'8.67" = 25.385742
    // EPSG:3067 N 7369564.333 -> EPSG:4326 latitude 66°26'13.646" = 66.437124
}
