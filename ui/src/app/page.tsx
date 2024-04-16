"use client";
import { toast } from "sonner";

import { Button } from "@/components/ui/button";
import { ConnectButton } from "@rainbow-me/rainbowkit";
import { getAccount } from "@wagmi/core";
import { config } from "./layout";

export default function Home() {
  const { status } = getAccount(config);

  // TODO: Switch to backend
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
      console.log("hi");
      // const res = await fetch("http://localhost:8000/api/prove");
      // const data = await res.json();
      // console.log(data);
    } catch (e) {
      console.log(e);
    }
  }

  return (
    <>
      <div className="flex flex-row justify-center items-center min-h-screen">
        {status === "disconnected" ? (
          <ConnectButton />
        ) : (
          <Button
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
            }}
          >
            Access and Prove location
          </Button>
        )}
      </div>
    </>
  );
}
