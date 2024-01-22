import { useGetProducts } from "@/hooks";
import React from "react";

const Product = () => {
  const { isLoading, data } = useGetProducts()();

  console.log("datadata", data);
  return <div>product</div>;
};

export default Product;
