import "./globals.css";
import React from 'react'
import { GiTomato } from "react-icons/gi";
import { FaTasks } from "react-icons/fa";
import { LuSettings } from "react-icons/lu";
import { TfiStatsUp } from "react-icons/tfi";
import { Link } from "wouter";


export default function RootLayout({ children }) {
  const iconSize = 120;
  return (
    <div className="bg-zinc-50">
      <nav className="">
        <ul className="flex justify-evenly justify-items-end text-2xl p-2 mb-32 font-bold h-4 border-xl">
          <Link key="/" href="/" >
            <GiTomato className="cursor-pointer" color="red" size={iconSize} />
          </Link>
          <Link key="/tasks" href="/tasks">
            <FaTasks className="cursor-pointer" color="teal" size={iconSize} />
          </Link>
          <Link key="/stats" href="/stats">
            <TfiStatsUp className="cursor-pointer" color="blue" size={iconSize} />
          </Link>
          <Link key="/settings" href="/settings">
            <LuSettings className="cursor-pointer" color="gray" size={iconSize} />
          </Link>
        </ul>
      </nav>
      {children}
    </div>
  );
}

