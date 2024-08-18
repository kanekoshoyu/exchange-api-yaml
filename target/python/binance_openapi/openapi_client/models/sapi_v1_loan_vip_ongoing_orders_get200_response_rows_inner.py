# coding: utf-8

"""
    Binance Public Spot API

    OpenAPI Specifications for the Binance Public Spot API  API documents:   - [https://github.com/binance/binance-spot-api-docs](https://github.com/binance/binance-spot-api-docs)   - [https://binance-docs.github.io/apidocs/spot/en](https://binance-docs.github.io/apidocs/spot/en)

    The version of the OpenAPI document: 1.0
    Generated by OpenAPI Generator (https://openapi-generator.tech)

    Do not edit the class manually.
"""  # noqa: E501


from __future__ import annotations
import pprint
import re  # noqa: F401
import json

from pydantic import BaseModel, ConfigDict, Field, StrictInt, StrictStr
from typing import Any, ClassVar, Dict, List, Optional
from typing import Optional, Set
from typing_extensions import Self

class SapiV1LoanVipOngoingOrdersGet200ResponseRowsInner(BaseModel):
    """
    SapiV1LoanVipOngoingOrdersGet200ResponseRowsInner
    """ # noqa: E501
    order_id: StrictInt = Field(alias="orderId")
    loan_coin: StrictStr = Field(alias="loanCoin")
    total_debt: StrictStr = Field(alias="totalDebt")
    residual_interest: StrictStr = Field(alias="residualInterest")
    collateral_account_id: StrictStr = Field(alias="collateralAccountId")
    collateral_coin: StrictStr = Field(alias="collateralCoin")
    collateral_value: StrictStr = Field(description="locked collateral value shown in USD value", alias="collateralValue")
    total_collateral_value_after_haircut: Optional[StrictStr] = Field(default=None, alias="totalCollateralValueAfterHaircut")
    locked_collateral_value: Optional[StrictStr] = Field(default=None, alias="lockedCollateralValue")
    current_ltv: StrictStr = Field(alias="currentLTV")
    expiration_time: StrictInt = Field(alias="expirationTime")
    loan_date: StrictStr = Field(alias="loanDate")
    loan_rate: StrictStr = Field(alias="loanRate")
    loan_term: StrictStr = Field(alias="loanTerm")
    __properties: ClassVar[List[str]] = ["orderId", "loanCoin", "totalDebt", "residualInterest", "collateralAccountId", "collateralCoin", "collateralValue", "totalCollateralValueAfterHaircut", "lockedCollateralValue", "currentLTV", "expirationTime", "loanDate", "loanRate", "loanTerm"]

    model_config = ConfigDict(
        populate_by_name=True,
        validate_assignment=True,
        protected_namespaces=(),
    )


    def to_str(self) -> str:
        """Returns the string representation of the model using alias"""
        return pprint.pformat(self.model_dump(by_alias=True))

    def to_json(self) -> str:
        """Returns the JSON representation of the model using alias"""
        # TODO: pydantic v2: use .model_dump_json(by_alias=True, exclude_unset=True) instead
        return json.dumps(self.to_dict())

    @classmethod
    def from_json(cls, json_str: str) -> Optional[Self]:
        """Create an instance of SapiV1LoanVipOngoingOrdersGet200ResponseRowsInner from a JSON string"""
        return cls.from_dict(json.loads(json_str))

    def to_dict(self) -> Dict[str, Any]:
        """Return the dictionary representation of the model using alias.

        This has the following differences from calling pydantic's
        `self.model_dump(by_alias=True)`:

        * `None` is only added to the output dict for nullable fields that
          were set at model initialization. Other fields with value `None`
          are ignored.
        """
        excluded_fields: Set[str] = set([
        ])

        _dict = self.model_dump(
            by_alias=True,
            exclude=excluded_fields,
            exclude_none=True,
        )
        return _dict

    @classmethod
    def from_dict(cls, obj: Optional[Dict[str, Any]]) -> Optional[Self]:
        """Create an instance of SapiV1LoanVipOngoingOrdersGet200ResponseRowsInner from a dict"""
        if obj is None:
            return None

        if not isinstance(obj, dict):
            return cls.model_validate(obj)

        _obj = cls.model_validate({
            "orderId": obj.get("orderId"),
            "loanCoin": obj.get("loanCoin"),
            "totalDebt": obj.get("totalDebt"),
            "residualInterest": obj.get("residualInterest"),
            "collateralAccountId": obj.get("collateralAccountId"),
            "collateralCoin": obj.get("collateralCoin"),
            "collateralValue": obj.get("collateralValue"),
            "totalCollateralValueAfterHaircut": obj.get("totalCollateralValueAfterHaircut"),
            "lockedCollateralValue": obj.get("lockedCollateralValue"),
            "currentLTV": obj.get("currentLTV"),
            "expirationTime": obj.get("expirationTime"),
            "loanDate": obj.get("loanDate"),
            "loanRate": obj.get("loanRate"),
            "loanTerm": obj.get("loanTerm")
        })
        return _obj


