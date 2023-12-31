'use client'
import React, { useLayoutEffect } from "react";
import { gsap } from "gsap";
import Image, { StaticImageData } from "next/image";
import { ScrollTrigger } from "gsap/ScrollTrigger";

gsap.registerPlugin(ScrollTrigger);

interface AnimateImageProps {
  src: string | StaticImageData,
  alt: string,
  triggerRef?: React.MutableRefObject<HTMLDivElement>,
  priority?: boolean,
  onScroll?: boolean,
  coverPlacement?: "top" | "center" | "bottom",
  position?: number,
  delay?: number,
  quality?: number; // Define quality as an optional number prop
  sizes?: string; // Define sizes as an optional string prop
}

const getCoverPlacement = (coverPlacement: "top" | "center" | "bottom") => {
  switch (coverPlacement) {
    case "top":
      return "object-top";
    case "center":
      return "object-center";
    case "bottom":
      return "object-bottom";
    default:
      return "object-center";
  }
}

export const AnimatedImage = ({
  src,
  alt,
  priority,
  onScroll,
  triggerRef,
  delay,
  position,
  quality, // Include the quality prop
  sizes, // Include the sizes prop
  coverPlacement,
}: AnimateImageProps) => {
  const imageRef = React.useRef(null);
  const computedTriggerRef = triggerRef || imageRef;
  var computedPosition = position || 0;

  useLayoutEffect(() => {
    let ctx = gsap.context(() => {
      const tl = onScroll ? gsap.timeline({
        scrollTrigger: {
          trigger: computedTriggerRef.current,
          onEnter: (self) => {
            gsap.set(imageRef.current, { visibility: "visible" });
            if (self.getVelocity() > 0) {
              computedPosition = 0;
            }
          },
        },
      }): gsap.timeline();

      const animation = {
        opacity: 0,
        y: 100,
        duration: 1,
        ease: "quint",
        delay: delay || 0,
      }

      tl.set(imageRef.current, { visibility: "visible" });

      tl.from(imageRef.current, animation, position || 0);
    });
    return () => {
      ctx.revert();
    };
  }, []);

  return (
    <Image
      src={src}
      quality={quality || 100} // Use the quality prop here with a default of 100
      style={{ padding: "inherit" }}
      className={`invisible object-cover ${getCoverPlacement(coverPlacement)}`}
      fill
      sizes="(max-width: 768px) 100vw, (max-width: 1200px) 100vw, 100vw"
      alt={alt}
      ref={imageRef}
      priority={priority}
      // placeholder="blur"
      // blurDataURL="encoded.txt"
    />
  );
};