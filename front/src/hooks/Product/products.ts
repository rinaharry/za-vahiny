import { useQuery } from "@tanstack/react-query";

import { URL_API } from "@/config";

const getProducts = async () => {
  const response = await fetch(URL_API);
  const data = await response.json();

  return data;
};

export const useGetProducts = () => {
  const Products = () => {
    const { data, isLoading } = useQuery({
      queryKey: ["todos"],
      queryFn: getProducts,
    });
    return { data, isLoading };
  };

  return Products;
};
