'use client'
import React, { useLayoutEffect } from "react";
import { gsap } from "gsap";
import { SplitText } from "gsap/SplitText";
import Image from 'next/image';

gsap.registerPlugin(SplitText);

export const LogoComponent = () => {
  const imgageRef = React.useRef(null);

  useLayoutEffect(() => {
    let ctx = gsap.context(() => {

      const tl = gsap.timeline();

      gsap.set(imgageRef.current, { perspective: 400 });
      gsap.set(imgageRef.current, { visibility: "visible" });

      tl.from(imgageRef.current, {
        delay: 2,
        duration: 1,
        opacity: 0,
        scale: 0,
        y: 80,
        rotationX: 180,
        transformOrigin: "0% 50% 50",
        ease: "expo.out",
        });    
    });
    return () => {
      ctx.revert();
    }
  }, []);

  return (
    <div  className="flex grow flex-row sm:justify-end sm:content-center sm:-order-none order-3">
      <div>
        <Image
          className="invisible"
          ref={imgageRef}
          src="/logo-igloo-white.png"
          width={40}
          height={40}
          alt="Logo of the company" />
      </div>
    </div>
  );
};
