import Link from 'next/link';
import Image from 'next/image';
import type { Product } from '@/lib/types';
import { PriceDisplay } from '@/components/PriceDisplay';

interface ProductCardProps {
  product: Product;
}

export function ProductCard({ product }: ProductCardProps) {
  const primaryImage = product.images.find((img) => img.is_primary) ?? product.images[0];

  return (
    <Link
      href={`/products/${product.id}`}
      className="card card-hover group block"
      aria-label={`View ${product.name}`}
    >
      <div className="relative aspect-[4/3] bg-neutral-100">
        {primaryImage ? (
          <Image
            src={primaryImage.url}
            alt={primaryImage.alt_text ?? product.name}
            fill
            sizes="(max-width: 640px) 100vw, (max-width: 1024px) 50vw, 25vw"
            className="object-cover transition-transform group-hover:scale-105"
          />
        ) : (
          <div className="flex h-full items-center justify-center bg-gradient-to-br from-neutral-100 to-neutral-200 text-neutral-400">
            <svg
              xmlns="http://www.w3.org/2000/svg"
              className="h-16 w-16"
              fill="none"
              viewBox="0 0 24 24"
              stroke="currentColor"
              aria-hidden="true"
            >
              <path
                strokeLinecap="round"
                strokeLinejoin="round"
                strokeWidth={1}
                d="M4 16l4.586-4.586a2 2 0 012.828 0L16 16m-2-2l1.586-1.586a2 2 0 012.828 0L20 14m-6-6h.01M6 20h12a2 2 0 002-2V6a2 2 0 00-2-2H6a2 2 0 00-2 2v12a2 2 0 002 2z"
              />
            </svg>
          </div>
        )}
        {product.compare_at_price != null && product.compare_at_price > product.price && (
          <span className="badge badge-error absolute left-2 top-2">Sale</span>
        )}
      </div>
      <div className="card-body">
        {product.category && (
          <span className="text-xs text-neutral-500 uppercase tracking-wide">
            {product.category.name}
          </span>
        )}
        <h3 className="mt-1 text-base font-medium text-neutral-900 group-hover:text-primary line-clamp-2">
          {product.name}
        </h3>
        <PriceDisplay
          price={product.price}
          compareAtPrice={product.compare_at_price}
          className="mt-2"
        />
      </div>
    </Link>
  );
}
