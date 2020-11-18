enum ShadingLocation {
    Interior,
    Exterior
}

enum ShadingClass {
    HorizontalBlinds,
    VerticalBlinds,
    Curtain
}

pub struct ShadingDevice {
    location: ShadingLocation,
    class: ShadingClass
}