"use client";
import Image from "next/image";
import styles from "./page.module.css";
import {
  useQuery,
  useMutation,
  useQueryClient,
  QueryClient,
  QueryClientProvider,
} from "@tanstack/react-query";
import { useGetProducts } from "@/hooks";
import Product from "@/components/Product/Product";

export default function Home() {
  // Create a client
  const queryClient = new QueryClient();

  return (
    <QueryClientProvider client={queryClient}>
      <Product />
    </QueryClientProvider>
  );
}
