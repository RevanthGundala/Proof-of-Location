import React from "react";

export default function GoogleMap({ location }: { location: string }) {
  const src = `https://www.google.com/maps/embed/v1/place?key=AIzaSyAG_BnN7PTxVwK07qYAoJdgff7jhsxCBV4&q=${location}`;
  return (
    <>
      <iframe width="600" height="450" loading="lazy" src={src}></iframe>
    </>
  );
}
