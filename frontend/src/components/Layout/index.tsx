import { Outlet } from "react-router-dom";
import ResponsivityFallback from "../ResponsivityFallback";
import { Toaster } from "../Toaster";

import Header from "./Header";

export default function Layout() {
  return (
    <div>
      <div className="md:invisible visible md:h-0">
        <ResponsivityFallback />
      </div>
      <div className="md:visible invisible h-screen flex flex-col">
        <Header />
        <div className="px-6 pb-6 flex-1">
          <Outlet />
          <Toaster />
        </div>
      </div>
    </div>
  );
}
