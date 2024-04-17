"use client";
import { toast } from "sonner";
import { useState } from "react";
import { Button } from "@/components/ui/button";
import { ConnectButton } from "@rainbow-me/rainbowkit";
import { getAccount } from "@wagmi/core";
import { config } from "./layout";
import GoogleMap from "@/components/ui/GoogleMap";
import { Input } from "@/components/ui/input";

export default function Home() {
  const [proved, setProved] = useState(false);
  const [distance, setDistance] = useState("1");
  const [location, setLocation] = useState("");
  const { status } = getAccount(config);
  console.log(status);
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
        body: JSON.stringify({ ip, location, distance }),
      });
      const data = await res.json();
      console.log(data);
    } catch (e) {
      console.log(e);
    }
  }

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
              placeholder="1"
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
            disabled={status !== "connected"}
            className="w-fit"
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

          {<GoogleMap location={location} />}
        </div>
        <div className="px-4 mt-10">
          <ConnectButton />
        </div>
      </div>
    </>
  );
}
