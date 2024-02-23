import type { Metadata } from "next";
import { Inter } from "next/font/google";
import "./globals.css";
import 'react-toastify/dist/ReactToastify.css';
import GlobalContext from "./Context/store";

const inter = Inter({ subsets: ["latin"] });

export const metadata: Metadata = {
  title: "Soroban",
  description: "",
  keywords:[],
};


export default function RootLayout({
  children,
}: Readonly<{
  children: React.ReactNode;
}>) {
  return (
    <html lang="en">

      <body className={inter.className}>
        <GlobalContext>
          {children}
        </GlobalContext>
      </body>
    </html>
  );
}
