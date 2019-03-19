import QtQuick 2.9
import QtQuick.Controls 2.2
import QtQuick.Layouts 1.3
import TicTacToe 1.0;

ApplicationWindow {
    visible: true
    width: 620
    height: 580
    color: "darkslategray"
    title: qsTr("Tic-tac-toe")
    
    function circle(ctx, x, y, width, height) {
        ctx.strokeStyle = "red";
        ctx.beginPath();
        ctx.ellipse(x, y, width, height);
        ctx.stroke();
    }

    function cross(ctx, x, y, width, height) {
        ctx.strokeStyle = "blue";
        ctx.beginPath();
        ctx.moveTo(x, y);
        ctx.lineTo(x+width, y+height);
        ctx.moveTo(x, y+height);
        ctx.lineTo(x+width, y);
        ctx.stroke();
    }

    function draw_square(ctx, square, x, y, width, height) {
        if(board[square] == 1)
            circle(ctx, x, y, width, height);
        else if(board[square] == 2)
            cross(ctx, x, y, width, height);
    }

    property var board: [0, 0, 0, 0, 0, 0, 0, 0, 0, 0];

    TicTacToe {
        id: game
        onDrawMove: {
            board[square] = player;
            gameCanvas.requestPaint();
        }
    }

    Canvas {
        id: gameCanvas
        anchors.fill: parent

        onPaint: {
            var ctx = getContext("2d");

            ctx.lineWidth = 5;
            ctx.strokeStyle = "white";
            ctx.beginPath();
            ctx.moveTo(gameCanvas.width*0.35, gameCanvas.height*0.1);
            ctx.lineTo(gameCanvas.width*0.35, gameCanvas.height*0.9);
            ctx.stroke();
            ctx.moveTo(gameCanvas.width*0.65, gameCanvas.height*0.1);
            ctx.lineTo(gameCanvas.width*0.65, gameCanvas.height*0.9);
            ctx.stroke();
            ctx.moveTo(gameCanvas.width*0.1, gameCanvas.height*0.35);
            ctx.lineTo(gameCanvas.width*0.9, gameCanvas.height*0.35);
            ctx.stroke();
            ctx.moveTo(gameCanvas.width*0.1, gameCanvas.height*0.65);
            ctx.lineTo(gameCanvas.width*0.9, gameCanvas.height*0.65);
            ctx.stroke();

            ctx.lineWidth = 5;
            ctx.strokeStyle = "red";
            var width = gameCanvas.width*0.18;
            var height = gameCanvas.height*0.18;
            var x = gameCanvas.width*0.13;
            var y = gameCanvas.height*0.13;
            
            draw_square(ctx, 0, x, y, width, height);
                        
            x=gameCanvas.width*0.41;
            draw_square(ctx, 1, x, y, width, height);

            x=gameCanvas.width*0.68;
            draw_square(ctx, 2, x, y, width, height);

            x = gameCanvas.width*0.13;
            y = gameCanvas.height*0.41;
            draw_square(ctx, 3, x, y, width, height);

            x=gameCanvas.width*0.41;
            draw_square(ctx, 4, x, y, width, height);

            x=gameCanvas.width*0.68;
            draw_square(ctx, 5, x, y, width, height);

            x = gameCanvas.width*0.13;
            y = gameCanvas.height*0.68;
            draw_square(ctx, 6, x, y, width, height);

            x=gameCanvas.width*0.41;
            draw_square(ctx, 7, x, y, width, height);

            x=gameCanvas.width*0.68;
            draw_square(ctx, 8, x, y, width, height);
        }
    }

    MouseArea {
        width: parent.width*0.35
        height: parent.height*0.35
        onClicked: {
            console.log("top left");
            game.doMove(0, 0);
        }
    }
    MouseArea {
        x: parent.width*0.35
        width: parent.width*0.3
        height: parent.height*0.35
        onClicked: {
            console.log("top");
            game.doMove(0, 1);
        }
    }
    MouseArea {
        x: parent.width*0.65
        width: parent.width - x
        height: parent.height*0.35
        onClicked: {
            console.log("top right");
            game.doMove(0, 2);
        }
    }
    MouseArea {
        y: parent.height*0.35
        width: parent.width*0.35
        height: parent.height*0.3
        onClicked: {
            console.log("left");
            game.doMove(1, 0);
        }
    }
    MouseArea {
        y: parent.height*0.35
        x: parent.width*0.35
        width: parent.width*0.3
        height: parent.height*0.3
        onClicked: {
            console.log("middle");
            game.doMove(1, 1);
        }
    }
    MouseArea {
        y: parent.height*0.35
        x: parent.width*0.65
        width: parent.width - x
        height: parent.height*0.3
        onClicked: {
            console.log("right");
            game.doMove(1, 2);
        }
    }
    MouseArea {
        y: parent.height*0.65
        width: parent.width*0.35
        height: parent.height-y
        onClicked: {
            console.log("bottom left");
            game.doMove(2, 0);
        }
    }
    MouseArea {
        y: parent.height*0.65
        x: parent.width*0.35
        width: parent.width*0.3
        height: parent.height-y
        onClicked: {
            console.log("bottom middle");
            game.doMove(2, 1);
        }
    }
    MouseArea {
        y: parent.height*0.65
        x: parent.width*0.65
        width: parent.width - x
        height: parent.height-y
        onClicked: {
            console.log("bottom right");
            game.doMove(2, 2);
        }
    }
}