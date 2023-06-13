'use client';

import React, { useState, useEffect } from 'react';
import { Game as GameType } from 'phaser';

const Game = () => {
  const [game, setGame] = useState<GameType>();

    useEffect(() => {
        async function initPhaser() {
            const Phaser = await import('phaser');

            const trafficGame = new Phaser.Game({
                type: Phaser.AUTO,
                title: 'Das Traffic Gamenheimer',
                parent: 'game-content',
                width: window.innerWidth,
                height: window.innerHeight,
                scene: {

                }
            })
        }
        initPhaser();
    }, []);

    return (
        <>
            <div id='game-content' key='game-content'></div>
        </>
    )
}