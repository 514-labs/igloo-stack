import React from "react";
import { AnimateImage } from "../../components/AnimateImage";
import { CodeBlockCTA } from "../../components/CodeBlockCTA";


export const ctaSection = {
  heading: "start building today",
  description: "Start building your data-intensive application today. Igloo is free to use and open source. If you'd like to contribute, check out our github or join our discord."

}

export const CTASection = () => {
  return (
    <div className=" pt-32 w-screen flex grow-1 flex-col my-24">
      <div className="h-full flex flex-col md:flex-row flex-grow md:justify-center md:items-center">
        <div className="text-white flex-col px-5 md:flex-1 ">
          <div className="px-5 text-5xl sm:text-6xl 2xl:text-9xl">
            {ctaSection.heading}
          </div>
          <div className="flex flex-col grow md:flex-1 p-5 space-y-5">
            <div className="text-typography-primary my-3">
              {ctaSection.description}
            </div>
            <div>
              <CodeBlockCTA />
            </div>
          </div>
        </div>
        <div className="flex flex-auto md:flex-1 flex-row  w-full md:justify-center md:items-center">
          <div className="flex w-full relative md:overflow-hidden ">
            <AnimateImage src="/hoodie.png" width={1024} height={1024} alt="developer in style" />
          </div>
        </div>
      </div>
    </div>
  );
};
