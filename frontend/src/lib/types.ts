// TypeScript types matching backend DTOs

export type ProductStatus = 'draft' | 'active' | 'archived';

export type ProductSort = 'price_asc' | 'price_desc' | 'created_at_desc' | 'name_asc';

export interface CategorySummary {
  id: string;
  name: string;
  slug: string;
}

export interface Category {
  id: string;
  name: string;
  slug: string;
  description: string | null;
  parent_id: string | null;
  sort_order: number;
  children: Category[];
  created_at: string;
  updated_at: string;
}

export interface ProductImage {
  id: string;
  product_id: string;
  url: string;
  alt_text: string | null;
  sort_order: number;
  is_primary: boolean;
  created_at: string;
}

export interface ProductVariant {
  id: string;
  product_id: string;
  name: string;
  sku: string | null;
  price: number;
  compare_at_price: number | null;
  stock_quantity: number;
  attributes: Record<string, unknown>;
  sort_order: number;
  is_active: boolean;
  created_at: string;
  updated_at: string;
}

export interface Product {
  id: string;
  name: string;
  slug: string;
  description: string | null;
  short_description: string | null;
  price: number;
  compare_at_price: number | null;
  status: ProductStatus;
  is_featured: boolean;
  sku: string | null;
  stock_quantity: number;
  attributes: Record<string, unknown>;
  category: CategorySummary | null;
  images: ProductImage[];
  variants: ProductVariant[];
  created_at: string;
  updated_at: string;
}

export interface PaginationMeta {
  page: number;
  per_page: number;
  total_items: number;
  total_pages: number;
}

export interface PaginatedResponse<T> {
  data: T[];
  pagination: PaginationMeta;
}

export interface ProductListParams {
  page?: number;
  per_page?: number;
  status?: ProductStatus;
  category_id?: string;
  sort?: ProductSort;
  q?: string;
}
