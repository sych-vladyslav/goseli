import { formatPrice } from '@/lib/api';

interface PriceDisplayProps {
  price: number;
  compareAtPrice?: number | null;
  className?: string;
}

export function PriceDisplay({ price, compareAtPrice, className }: PriceDisplayProps) {
  const hasDiscount = compareAtPrice != null && compareAtPrice > price;

  return (
    <div className={`flex items-center gap-2 ${className ?? ''}`}>
      <span
        className={`font-semibold ${hasDiscount ? 'text-error' : 'text-neutral-900'}`}
      >
        {formatPrice(price)}
      </span>
      {hasDiscount && (
        <span className="text-sm text-neutral-400 line-through">
          {formatPrice(compareAtPrice)}
        </span>
      )}
    </div>
  );
}
