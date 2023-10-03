'use client'
import React, { useLayoutEffect } from "react";
import { AnimateImage } from "../../components/AnimateImage";
import { CodeBlockCTA } from "../../components/CodeBlockCTA";
import { gsap } from "gsap";
import { SplitText } from "gsap/SplitText";
import { ScrollTrigger } from "gsap/ScrollTrigger";

export const heroContent = {
  heading: "the application framework for the modern data stack",
  description: "Igloo is a batteries-included framework for building data-intensive applications using Typescript or Python, and SQL. It comes with a powerful CLI to help automate development tasks, an intuitive abstraction to help you build quickly, and a streamlined local development workflow."
}

export const HeroSection = () => {
  const headingRef = React.useRef(null);
  const descriptionRef = React.useRef(null)

  useLayoutEffect(() => {
    let ctx = gsap.context(() => {

      const tl = gsap.timeline();
      const splitText = new SplitText(headingRef.current, { type: "words, chars" });
      const splitTextChars = splitText.chars;

      const splitTextByLines = new SplitText(descriptionRef.current, {type: "lines"});
      const splitTextLines = splitTextByLines.lines;

      gsap.set(headingRef.current, { visibility: "visible" });
      gsap.set(descriptionRef.current, {visibility: "visible"})

      tl.from(splitTextChars,{
        y: "-20",
        opacity: 0,
        ease: "quint",
        stagger: { each: 0.03 },
        }, 0);

      tl.from(splitTextLines,{
        y: "-10",
        opacity: 0,
        ease: "quint",
        stagger: { each: 0.03 },
        }, 1);
    });
    return () => {
      ctx.revert();
    }
  }, []);


  return <div className=" w-screen flex grow-1 flex-col pt-24 mb-24">
    <div className="h-full flex flex-col md:flex-row flex-grow md:justify-center md:items-center">
      <div className="text-white flex-col px-5 md:flex-1 ">
        <div className="px-5 text-5xl sm:text-6xl 2xl:text-9xl">
        <span  ref={headingRef} className="invisible">
          {heroContent.heading}
          </span>
        </div>
        <div className="flex flex-col grow md:flex-1 p-5 space-y-5">
          <div className="text-typography-primary my-3">
            <span className="invisible" ref={descriptionRef}>
            {heroContent.description}
            </span>
          </div>
          <div>
            <CodeBlockCTA />
          </div>
        </div>
      </div>
      <div className="flex flex-auto md:flex-1 flex-row md:h-full w-full md:justify-center md:items-center mt-24">
        <div className="flex w-full relative md:overflow-hidden ">
          <AnimateImage src="/hero.png" width={1024} height={1024} alt="developer lifestyle" priority />
        </div>
      </div>
    </div>
  </div>;
};