'use client';

import useSWR from 'swr';
import type {
  Product,
  PaginatedResponse,
  ProductListParams,
  Category,
} from '@/lib/types';
import { fetchApi } from '@/lib/api';

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
