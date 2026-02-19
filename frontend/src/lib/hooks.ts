'use client';

import useSWR from 'swr';
import type {
  Product,
  PaginatedResponse,
  ProductListParams,
  Category,
  CartResponse,
  AddToCartRequest,
  UpdateCartItemRequest,
} from '@/lib/types';
import { fetchApi, addToCart as apiAddToCart, updateCartItem as apiUpdateCartItem, removeCartItem as apiRemoveCartItem, clearCart as apiClearCart } from '@/lib/api';

function buildQueryString(params: Record<string, unknown>): string {
  const searchParams = new URLSearchParams();
  for (const [key, value] of Object.entries(params)) {
    if (value !== undefined && value !== null && value !== '') {
      searchParams.set(key, String(value));
    }
  }
  const qs = searchParams.toString();
  return qs ? `?${qs}` : '';
}

const clientFetcher = <T>(url: string) => fetchApi<T>(url);

export function useProducts(params: ProductListParams = {}) {
  const qs = buildQueryString(params as Record<string, unknown>);
  return useSWR<PaginatedResponse<Product>>(
    `/api/v1/products${qs}`,
    clientFetcher,
    { keepPreviousData: true },
  );
}

export function useProduct(id: string | null) {
  return useSWR<Product>(
    id ? `/api/v1/products/${id}` : null,
    clientFetcher,
  );
}

export function useCategories() {
  return useSWR<Category[]>(
    '/api/v1/categories',
    clientFetcher,
  );
}

export function useCart() {
  const { data, error, isLoading, mutate } = useSWR<CartResponse>(
    '/api/v1/cart',
    (url: string) => fetchApi<CartResponse>(url, { credentials: 'include' }),
  );

  const addToCart = async (request: AddToCartRequest) => {
    const updatedCart = await apiAddToCart(request);
    await mutate(updatedCart, false);
    return updatedCart;
  };

  const updateCartItem = async (itemId: string, request: UpdateCartItemRequest) => {
    const updatedCart = await apiUpdateCartItem(itemId, request);
    await mutate(updatedCart, false);
    return updatedCart;
  };

  const removeCartItem = async (itemId: string) => {
    await apiRemoveCartItem(itemId);
    await mutate();
  };

  const clearCart = async () => {
    await apiClearCart();
    await mutate();
  };

  return {
    cart: data,
    isLoading,
    error,
    addToCart,
    updateCartItem,
    removeCartItem,
    clearCart,
  };
}
