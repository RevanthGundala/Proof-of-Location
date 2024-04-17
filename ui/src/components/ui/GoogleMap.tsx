import React from "react";
import { APIProvider, Map } from "@vis.gl/react-google-maps";

export default function GoogleMap({ lat, lng }: { lat: number; lng: number }) {
  return (
    <APIProvider apiKey={"AIzaSyAG_BnN7PTxVwK07qYAoJdgff7jhsxCBV4"}>
      <Map zoom={15} center={{ lat, lng }} />
    </APIProvider>
  );
}
