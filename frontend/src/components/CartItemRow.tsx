'use client';

import Image from 'next/image';
import Link from 'next/link';
import { useState } from 'react';
import type { CartItemResponse } from '@/lib/types';
import { formatPrice } from '@/lib/api';
import { QuantitySelector } from './QuantitySelector';

interface CartItemRowProps {
  item: CartItemResponse;
  onUpdateQuantity: (itemId: string, quantity: number) => Promise<void>;
  onRemove: (itemId: string) => Promise<void>;
}

export function CartItemRow({ item, onUpdateQuantity, onRemove }: CartItemRowProps) {
  const [isRemoving, setIsRemoving] = useState(false);

  const handleRemove = async () => {
    setIsRemoving(true);
    try {
      await onRemove(item.id);
    } catch (error) {
      console.error('Failed to remove item:', error);
      setIsRemoving(false);
    }
  };

  const handleQuantityChange = async (newQuantity: number) => {
    await onUpdateQuantity(item.id, newQuantity);
  };

  return (
    <div className="flex gap-4 py-4 border-b border-neutral-200 last:border-b-0">
      <Link
        href={`/products/${item.product_id}`}
        className="relative flex-shrink-0 w-20 h-20 sm:w-24 sm:h-24 bg-neutral-100 rounded-md overflow-hidden hover:opacity-80 transition-opacity"
      >
        {item.product_image_url ? (
          <Image
            src={item.product_image_url}
            alt={item.product_name}
            fill
            sizes="(max-width: 640px) 80px, 96px"
            className="object-cover"
          />
        ) : (
          <div className="flex h-full items-center justify-center bg-gradient-to-br from-neutral-100 to-neutral-200">
            <svg
              xmlns="http://www.w3.org/2000/svg"
              className="h-8 w-8 text-neutral-400"
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
      </Link>

      <div className="flex-1 min-w-0">
        <div className="flex items-start justify-between gap-4">
          <div className="flex-1 min-w-0">
            <Link
              href={`/products/${item.product_id}`}
              className="font-medium text-neutral-900 hover:text-primary line-clamp-2"
            >
              {item.product_name}
            </Link>
            {item.variant_name && (
              <p className="text-sm text-neutral-500 mt-1">{item.variant_name}</p>
            )}
          </div>
          <p className="font-semibold text-neutral-900 flex-shrink-0">
            {formatPrice(item.subtotal)}
          </p>
        </div>

        <div className="flex items-center justify-between gap-4 mt-3">
          <QuantitySelector
            value={item.quantity}
            onChange={handleQuantityChange}
            disabled={isRemoving}
          />
          <div className="flex items-center gap-2">
            <span className="text-sm text-neutral-500 hidden sm:inline">
              {formatPrice(item.price)} each
            </span>
            <button
              onClick={handleRemove}
              disabled={isRemoving}
              className="text-sm text-error hover:text-error-dark disabled:opacity-40 transition-colors"
              aria-label={`Remove ${item.product_name} from cart`}
            >
              {isRemoving ? 'Removing...' : 'Remove'}
            </button>
          </div>
        </div>
      </div>
    </div>
  );
}
