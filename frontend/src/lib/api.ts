import type {
  Product,
  PaginatedResponse,
  ProductListParams,
  Category,
  CartResponse,
  AddToCartRequest,
  UpdateCartItemRequest,
} from '@/lib/types';

export class ApiError extends Error {
  constructor(
    public status: number,
    message: string,
  ) {
    super(message);
    this.name = 'ApiError';
  }
}

export async function fetchApi<T>(
  path: string,
  options?: RequestInit,
): Promise<T> {
  const res = await fetch(path, {
    ...options,
    headers: {
      'Content-Type': 'application/json',
      ...options?.headers,
    },
  });

  if (!res.ok) {
    const body = await res.text();
    throw new ApiError(res.status, body || res.statusText);
  }

  return res.json() as Promise<T>;
}

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

const API_BASE = process.env.NEXT_PUBLIC_API_URL || 'http://localhost:3001';

export async function getProducts(
  params: ProductListParams = {},
): Promise<PaginatedResponse<Product>> {
  const qs = buildQueryString(params as Record<string, unknown>);
  return fetchApi<PaginatedResponse<Product>>(
    `${API_BASE}/api/v1/products${qs}`,
  );
}

export async function getProduct(id: string): Promise<Product> {
  return fetchApi<Product>(`${API_BASE}/api/v1/products/${id}`);
}

export async function getCategories(): Promise<Category[]> {
  return fetchApi<Category[]>(`${API_BASE}/api/v1/categories`);
}

export function formatPrice(cents: number): string {
  return new Intl.NumberFormat('en-US', {
    style: 'currency',
    currency: 'USD',
  }).format(cents / 100);
}

export async function getCart(): Promise<CartResponse> {
  return fetchApi<CartResponse>(`${API_BASE}/api/v1/cart`, {
    credentials: 'include',
  });
}

export async function addToCart(
  request: AddToCartRequest,
): Promise<CartResponse> {
  return fetchApi<CartResponse>(`${API_BASE}/api/v1/cart/items`, {
    method: 'POST',
    body: JSON.stringify(request),
    credentials: 'include',
  });
}

export async function updateCartItem(
  itemId: string,
  request: UpdateCartItemRequest,
): Promise<CartResponse> {
  return fetchApi<CartResponse>(`${API_BASE}/api/v1/cart/items/${itemId}`, {
    method: 'PUT',
    body: JSON.stringify(request),
    credentials: 'include',
  });
}

export async function removeCartItem(itemId: string): Promise<void> {
  await fetch(`${API_BASE}/api/v1/cart/items/${itemId}`, {
    method: 'DELETE',
    credentials: 'include',
  });
}

export async function clearCart(): Promise<void> {
  await fetch(`${API_BASE}/api/v1/cart`, {
    method: 'DELETE',
    credentials: 'include',
  });
}
