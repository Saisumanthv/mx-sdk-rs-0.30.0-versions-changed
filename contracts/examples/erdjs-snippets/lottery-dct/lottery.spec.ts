import { Balance, Token, TokenType } from "@dharitrinetwork/erdjs";
import { AirdropService, createTokenAmount, DCTInteractor, ITestSession, IUser, TestSession } from "@dharitrinetwork/erdjs-snippets";
import { assert } from "chai";
import { LotteryInteractor } from "./lotteryInteractor";

describe("lottery snippet", async function () {
    this.bail(true);

    const LotteryName = "fooLottery";

    let suite = this;
    let session: ITestSession;
    let whale: IUser;
    let owner: IUser;

    this.beforeAll(async function () {
        session = await TestSession.loadOnSuite("default", suite);
        whale = session.users.whale;
        owner = session.users.alice;
        await session.syncNetworkConfig();
    });

    it("airdrop MOAX", async function () {
        session.expectLongInteraction(this);

        let amount = Balance.moax(1);
        await session.syncUsers([whale]);
        await AirdropService.createOnSession(session).sendToEachUser(whale, amount);
    });

    it("issue lottery token", async function () {
        session.expectLongInteraction(this);

        let interactor = await DCTInteractor.create(session);
        let token = new Token({ name: "FOO", ticker: "FOO", decimals: 0, supply: "100000000", type: TokenType.Fungible })
        await session.syncUsers([owner]);
        await interactor.issueToken(owner, token);
        await session.saveToken("lotteryToken", token);
    });

    it("airdrop lottery token", async function () {
        session.expectLongInteraction(this);

        let lotteryToken = await session.loadToken("lotteryToken");
        let amount = createTokenAmount(lotteryToken, "10");
        await session.syncUsers([owner]);
        await AirdropService.createOnSession(session).sendToEachUser(owner, amount);
    });

    it("setup", async function () {
        session.expectLongInteraction(this);

        await session.syncUsers([owner]);

        let interactor = await LotteryInteractor.create(session);
        let contractAddress = await interactor.deploy(owner);
        await session.saveAddress("contractAddress", contractAddress);
    });

    it("start lottery", async function () {
        session.expectLongInteraction(this);

        await session.syncUsers([owner]);

        let contractAddress = await session.loadAddress("contractAddress");
        let lotteryToken = await session.loadToken("lotteryToken");
        let interactor = await LotteryInteractor.create(session, contractAddress);
        await interactor.start(owner, LotteryName, lotteryToken, 1);
    });

    it("get lottery info and status", async function () {
        let contractAddress = await session.loadAddress("contractAddress");
        let lotteryToken = await session.loadToken("lotteryToken");
        let interactor = await LotteryInteractor.create(session, contractAddress);
        let lotteryInfo = await interactor.getLotteryInfo(owner, LotteryName);
        let lotteryStatus = await interactor.getStatus(owner, LotteryName);
        console.log("Info:", lotteryInfo);
        console.log("Prize pool:", lotteryInfo.prize_pool.toString());
        console.log("Status:", lotteryStatus);

        assert.equal(lotteryInfo.token_identifier.toString(), lotteryToken.identifier);
        assert.equal(lotteryStatus, "Running");
    });

    it("friends buy tickets", async function () {
        session.expectLongInteraction(this);

        await session.syncAllUsers();

        let contractAddress = await session.loadAddress("contractAddress");
        let lotteryToken = await session.loadToken("lotteryToken");
        let interactor = await LotteryInteractor.create(session, contractAddress);
        
        let buyAmount = createTokenAmount(lotteryToken, "1");
        let buyPromises = session.users.getFriends().map(friend => interactor.buyTicket(friend, LotteryName, buyAmount));
        await Promise.all(buyPromises);
    });
});
