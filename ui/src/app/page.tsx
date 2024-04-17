"use client";
import { toast } from "sonner";
import { useEffect, useState } from "react";
import { Button } from "@/components/ui/button";
import { ConnectButton } from "@rainbow-me/rainbowkit";
import { config } from "./layout";
import { useAccount } from "wagmi";
import GoogleMap from "@/components/ui/GoogleMap";
import { Input } from "@/components/ui/input";

export default function Home() {
  const [proved, setProved] = useState(false);
  const [distance, setDistance] = useState("1.0");
  const [location, setLocation] = useState("United States");
  const [latitude, setLatitude] = useState(0);
  const [longitude, setLongitude] = useState(0);

  const account = useAccount({
    config,
  });
  async function getIPAddress() {
    try {
      console.log("Fetching IP Address");
      const res = await fetch("https://api.ipify.org/?format=json");
      const data = await res.json();
      console.log(data);
      return data.ip;
    } catch (e) {
      console.log(e);
    }
  }

  async function prove(e: any) {
    e.preventDefault();
    try {
      const ip = await getIPAddress();
      const res = await fetch("http://localhost:8080/api/prove", {
        method: "POST",
        headers: {
          "Content-Type": "application/json",
        },
        body: JSON.stringify({ ip, longitude, latitude, distance }),
      });
      const data = await res.text();
      console.log(data);
    } catch (e) {
      console.log(e);
    }
  }

  useEffect(() => {
    console.log(location);
    fetch(
      `https://maps.googleapis.com/maps/api/geocode/json?address=${location}&key=AIzaSyAG_BnN7PTxVwK07qYAoJdgff7jhsxCBV4`
    )
      .then((response) => response.json())
      .then((locationData) => {
        const { lat, lng } = locationData.results[0].geometry.location;
        setLatitude(lat);
        setLongitude(lng);
      })
      .catch((e) => console.log(e));
  }, [location]);

  return (
    <>
      <div className="flex flex-row flex-1">
        <div className="flex flex-col flex-grow mt-10 space-y-8 min-h-screen px-10">
          <h1 className="font-semibold text-3xl">Proof of Location</h1>
          <h2 className="mb-20">
            Enter an Address and Generate a Proof that you are within {distance}{" "}
            mile(s) of that area.
          </h2>

          <div className="flex flex-col space-y-6">
            <Input
              className="w-1/4"
              type="Miles"
              placeholder="1.0"
              onChange={(e) => setDistance(e.target.value)}
            />
            <Input
              className="w-1/2"
              type="location"
              placeholder="Location"
              onChange={(e) => setLocation(e.target.value)}
            />
          </div>

          <Button
            disabled={account.status !== "connected"}
            className="w-fit disabled:cursor-not-allowed disabled:opacity-50"
            variant="outline"
            onClick={async (e) => {
              await prove(e);
              toast("Proof has been created", {
                description: "Sunday, December 03, 2023 at 9:00 AM",
                action: {
                  label: "Close",
                  onClick: () => console.log("Closed toast"),
                },
              });
              setProved(true);
            }}
          >
            Prove
          </Button>

          {
            <GoogleMap
              lat={latitude}
              lng={longitude}
              distance={parseFloat(distance)}
            />
          }
        </div>
        <div className="px-4 mt-10">
          <ConnectButton />
        </div>
      </div>
    </>
  );
}
