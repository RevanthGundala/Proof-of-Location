import React, { useEffect, useRef } from "react";
import { APIProvider, Map, Marker, useMap } from "@vis.gl/react-google-maps";

interface LatLng {
  lat: number;
  lng: number;
}

const CircleComponent = ({
  center,
  radius,
}: {
  center: LatLng;
  radius: number;
}) => {
  const map = useMap();

  useEffect(() => {
    if (map) {
      // Creating a new circle overlay
      const circle = new window.google.maps.Circle({
        map: map,
        center: center,
        radius: radius,
        fillColor: "red",
        fillOpacity: 0.5,
        strokeColor: "red",
        strokeOpacity: 0.8,
        strokeWeight: 2,
      });

      return () => {
        // Cleanup the circle from the map
        circle.setMap(null);
      };
    }
  }, [map, center, radius]);

  return null; // This component does not render anything itself
};

export default function GoogleMap({
  lat,
  lng,
  distance,
}: {
  lat: number;
  lng: number;
  distance: number;
}) {
  const center = { lat, lng };
  const radius = distance * 1609.34; // Convert miles to meters

  return (
    <APIProvider apiKey="AIzaSyAG_BnN7PTxVwK07qYAoJdgff7jhsxCBV4">
      <Map zoom={10} center={center}>
        <Marker position={center} />
        <CircleComponent center={center} radius={radius} />
      </Map>
    </APIProvider>
  );
}
